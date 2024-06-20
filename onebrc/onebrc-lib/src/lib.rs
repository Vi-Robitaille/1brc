use bstr::ByteSlice;
use gxhash::{HashMap, HashMapExt};
use memmap2::Mmap;
use std::collections::BTreeMap;
use std::fs::File;
use std::ops::Range;
use std::sync::OnceLock;
use std::thread::available_parallelism;
use std::thread::{self, JoinHandle};

mod data_types;
use data_types::*;

const FILE_NAME: &str = "../measurements.txt";
pub static MMAP: OnceLock<Mmap> = OnceLock::new();

pub fn init_mmap(file_name: Option<&str>) {
    let file = File::open(file_name.unwrap_or(FILE_NAME)).unwrap();
    MMAP.get_or_init(|| unsafe { Mmap::map(&file).unwrap() });
}

pub fn make_me_the_good_good(print: bool) {
    let available_cores: usize = available_parallelism().unwrap().into();

    let mut indexes: Vec<usize> = vec![0];
    indexes.extend(
        (1..available_cores)
            .map(|x| {
                let idx = x * (MMAP.get().unwrap().len() / available_cores);
                nearby_search(&MMAP.get().unwrap(), (idx - 31)..(idx + 32)).unwrap()
            })
            .collect::<Vec<usize>>(),
    );
    indexes.push(MMAP.get().unwrap().len());
    let ranges = indexes
        .windows(2)
        .map(|a| a[0]..a[1])
        .collect::<Vec<Range<usize>>>();

    // let (tx, rx) = channel();
    let mut thread_handles: Vec<JoinHandle<_>> = Vec::with_capacity(available_cores);
    for r in ranges {
        let thread_handle = process_chunk(r);
        thread_handles.push(thread_handle);
    }

    // drop(tx);
    let mut final_map = BTreeMap::new();
    for (idx, th) in thread_handles.into_iter().enumerate() {
        let thread_map = th.join().unwrap();
        if idx == 0 {
            final_map.extend(thread_map);
        } else {
            for (key, value) in thread_map.into_iter() {
                final_map
                    .entry(key)
                    .and_modify(|x| {
                        x.min = NumberTypeUsed::min(x.min, value.min);
                        x.max = NumberTypeUsed::max(x.max, value.max);
                        x.count += value.count;
                        x.sum += value.sum;
                    })
                    .or_insert(value);
            }
        }
    }

    if print {
        for (idx, (key, value)) in final_map.iter().enumerate() {
            if idx == 0 {
                print!("{{ ");
            } else {
                print!(", ");
            }
            let city = key.as_bstr();
            print!("\"{city}\": {value}");
        }
        print!("}}");
    }
}

// #[inline]
fn process_chunk(range: Range<usize>) -> JoinHandle<HashMap<Box<[u8]>, WeatherInfo>> {
    let thread_handle = thread::spawn(move || {
        let mut hm: HashMap<Box<[u8]>, WeatherInfo> = HashMap::new();
        let mut idx = range.start;
        while range.contains(&idx) {
            // finds the ; character which specifies the start of the number and end of the name
            let name_end =
                if let Some(v) = MMAP.get().unwrap()[idx..].iter().position(|&x| x == 0x3B) {
                    idx + v
                } else {
                    return HashMap::new();
                };

            let value_end = if let Some(v) = MMAP.get().unwrap()[name_end..]
                .iter()
                .position(|&x| x == 0x0A)
            {
                name_end + v
            } else {
                return HashMap::new();
            };

            // let name = StringUnion::from_utf8(&MMAP.get().unwrap()[idx..name_end]);
            let name = &MMAP.get().unwrap()[idx..name_end];
            let boxed = Box::<[u8]>::from(name);
            let value = WeatherInfo::new(&MMAP.get().unwrap()[(name_end + 1)..value_end]);
            *hm.entry(boxed).or_default() += value;
            idx = value_end + 1;
        }
        hm
    });
    thread_handle
}

// Returns the nearest \n (0x0A) found
#[inline]
fn nearby_search(mmap: &Mmap, range: Range<usize>) -> Option<usize> {
    for i in range.rev() {
        if mmap[i] == 0x0A {
            return Some(i + 1);
        }
    }
    None
}
