use gxhash::{HashMap, HashMapExt};
use itertools::Itertools;
use memmap2::Mmap;
use std::fs::File;
use std::ops::AddAssign;
use std::sync::OnceLock;

const FILE_NAME: &str = "../measurements.txt";
pub static MMAP: OnceLock<Mmap> = OnceLock::new();
type NumberTypeUsed = i32;

#[cfg_attr(rustfmt, rustfmt_skip)]
const NUMBER_LOOKUP: [NumberTypeUsed; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,-1, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];


pub fn init_mmap() {
    let file = File::open(FILE_NAME).unwrap();
    MMAP.get_or_init(|| unsafe { Mmap::map(&file).unwrap() });
}

#[allow(unused)]
trait OneBRCSplitIter: Iterator {
    /// Splits an iterator into a left and right hand side based on the first found `key` exclusively
    ///
    /// ```
    /// let data = vec![1, 3, -2, 1, 0, 1, 2];
    /// // chunks:     |----|    |----------|
    ///
    /// // Note: The `&` is significant here, `ChunkBy` is iterable
    /// // only by reference. You can also call `.into_iter()` explicitly.
    /// let mut data_grouped = Vec::new();
    /// for chunk in &data.into_iter().split_by_key(-2) {
    ///     data_grouped.push(chunk);
    /// }
    /// assert_eq!(data_grouped, vec![vec![1, 3], vec![1, 0, 1, 2]]);
    /// ```
    fn split_by_key(&mut self, key: Self::Item) -> (Vec<Self::Item>, Vec<Self::Item>)
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        let mut a: Vec<Self::Item> = vec![];
        let mut b: Vec<Self::Item> = vec![];
        let mut found: bool = false;
        while let Some(x) = self.next() {
            if x == key {
                found = true;
                continue;
            }
            if !found {
                a.push(x);
            } else {
                b.push(x)
            }
        }
        (a, b)
    }
}

#[derive(Debug, Clone)]
struct WeatherInfo {
    sum: NumberTypeUsed,
    min: NumberTypeUsed,
    max: NumberTypeUsed,
    count: NumberTypeUsed,
}

impl WeatherInfo {
    fn new(bytes: &[u8]) -> Self {
        let value: NumberTypeUsed = bytes_to_number_used(bytes);
        WeatherInfo {
            sum: value,
            min: value,
            max: value,
            count: 1,
        }
    }

    fn add_element(&mut self, bytes: &[u8]) {
        self.count += 1;
        let num = bytes_to_number_used(bytes);
        self.min = NumberTypeUsed::min(self.min, num);
        self.max = NumberTypeUsed::max(self.max, num);
        self.sum += num;
    }
}

impl Default for WeatherInfo {
    fn default() -> Self {
        WeatherInfo {
            sum: 0,
            min: 99999999,
            max: -99999999,
            count: 0,
        }
    }
}

impl AddAssign for WeatherInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.min = NumberTypeUsed::min(self.min, rhs.min);
        self.max = NumberTypeUsed::max(self.max, rhs.max);
        self.count += rhs.count;
        self.sum += rhs.sum;
    }
}

impl std::fmt::Display for WeatherInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{'min': {:.1}, 'mean': {:.1}, 'max': {:.1} }},",
            (self.min as f32) / 10_f32,
            (self.sum as f32) / (self.count as f32),
            (self.max as f32) / 10_f32
        )
    }
}

#[inline]
fn bytes_to_number_used(bytes: &[u8]) -> NumberTypeUsed {
    let sign: NumberTypeUsed = if bytes[0] == 0x2D { -1 } else { 1 };
    let number: NumberTypeUsed = bytes
        .iter()
        .rev()
        .filter(|&e| e != &0x2E && e != &0x2D)
        .enumerate()
        .map(|(i, &e)| {
            let pos = NumberTypeUsed::from(10).pow(u32::try_from(i).unwrap());
            NUMBER_LOOKUP[e as usize] * pos
        })
        .reduce(|a, b| a + b)
        .unwrap();
    number * sign
}

#[allow(unused)]
fn doodoofard() {
    let file = File::open(FILE_NAME).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };

    let mut hm: HashMap<String, WeatherInfo> = HashMap::with_capacity(10000);
    let _ = &mmap
        .iter()
        .chunk_by(|x| x == &&0x0A)
        .into_iter()
        .filter_map(|(key, chunk)| (!key).then_some(chunk))
        .for_each(|c| {
            // split
            let i = c.chunk_by(|x| x == &&0x3B);
            let mut i = i
                .into_iter()
                .filter_map(|(key, chunk)| (!key).then_some(chunk.copied().collect()));

            // Probably a dumb copy
            let name = String::from_utf8(i.next().unwrap()).expect("Wow! it failed!");
            let number = i.next().unwrap();
            hm.entry(name)
                .and_modify(|e| e.add_element(&number))
                .or_insert(WeatherInfo::new(&number));
        });
    println!("{}", hm.keys().len());
    for (k, v) in hm.iter() {
        println!("'{k}': {v}")
    }
}

#[allow(unused)]
fn low_optim() {

    // Low optim code
    // let _ = &mmap
    //     .iter()
    //     .chunk_by(|x| x == &&0x0A)
    //     .into_iter()
    //     .filter_map(|(key, chunk)| (!key).then_some(chunk) )
    //     .map(|mut chunk| {
    //         let mut name_buf: Vec<u8> = vec![];
    //         let mut value_buf: Vec<u8> = vec![];
    //         let mut found_split: bool = false;
    //         while let Some(x) = chunk.next() {
    //             match (x, found_split) {
    //                 (&0x3B, false) => { found_split = true },
    //                 (n, true) => { value_buf.push(*n) },
    //                 (n, false) => { name_buf.push(*n) }
    //             }
    //         }
    //         (String::from_utf8(name_buf).unwrap(), WeatherInfo2::new(&value_buf))
    //     })
    //     .for_each(|x| {
    //         hm.entry(x.0).and_modify(|w| *w += x.1.clone()).or_insert(x.1);
    //     });
}

pub mod statemachine {
    use gxhash::{HashMap, HashMapExt};
    use memmap2::Mmap;
    use std::ops::Range;
    use std::sync::mpsc::{channel, Sender};
    use std::thread::available_parallelism;
    use std::thread::{self, JoinHandle};
    use crate::WeatherInfo;
    use crate::MMAP;

    pub fn make_me_the_good_good(print: bool) {
        // is it faster to have each thread manage its own hashmap then write it all back at the end? or do it as we do now

        let available_parallelism: usize = available_parallelism().unwrap().into();
        // let available_parallelism: usize = 2;
        // println!("{}", mmap.len());

        let mut indexes: Vec<usize> = vec![0];
        indexes.extend(
            (1..available_parallelism)
                .map(|x| {
                    let idx = x * (MMAP.get().unwrap().len() / available_parallelism);
                    nearby_search(&MMAP.get().unwrap(), (idx - 31)..(idx + 32)).unwrap()
                })
                .collect::<Vec<usize>>(),
        );
        indexes.push(MMAP.get().unwrap().len());
        // println!("{:?}", indexes);

        let ranges = indexes
            .windows(2)
            .map(|a| a[0]..a[1])
            .collect::<Vec<Range<usize>>>();

        // println!("{:?}", ranges);

        let (tx, rx) = channel();

        let mut thread_handles: Vec<JoinHandle<_>> = Vec::with_capacity(available_parallelism);

        for r in ranges {
            let tx = tx.clone();
            let thread_handle = process_chunk(tx, r);
            thread_handles.push(thread_handle);
        }

        for th in thread_handles.into_iter() {
            let _ = th.join();
        }

        drop(tx);
        let _result = thread::scope(|scope| {
            let mut hm: HashMap<_, crate::WeatherInfo> = HashMap::with_capacity(10000);
            scope
                .spawn(move || {
                    let memmap_handle = MMAP.get().unwrap();
                    while let Ok(vec_of_data) = rx.recv() {
                        for (name_range, data_range) in vec_of_data {
                            let name = StringUnion { bytes: &memmap_handle[name_range] };
                            let value = WeatherInfo::new(&memmap_handle[data_range]);
                            *hm.entry(unsafe { name.string }).or_default() += value;
                        }
                    }
                    hm
                }).join().unwrap()
        });
        if print {
            println!("{:?}", _result);
        }
    }

    // #[inline]
    fn process_chunk(tx: Sender<Vec<(Range<usize>, Range<usize>)>>, range: Range<usize>) -> JoinHandle<()> {

        let thread_handle = thread::spawn(move || {
            let mut idx = range.start;
            let mut results = vec![];
            while range.contains(&idx) {
                // finds the ; character which specifies the start of the number and end of the name
                let name_end =
                    if let Some(v) = MMAP.get().unwrap()[idx..].iter().position(|&x| x == 0x3B) {
                        idx + v
                    } else {
                        return;
                    };

                let value_end =
                    if let Some(v) = MMAP.get().unwrap()[name_end..].iter().position(|&x| x == 0x0A) {
                        name_end + v
                    } else {
                        return;
                    };

                let name = idx..name_end;
                let value = (name_end + 1)..value_end;
                results.push((name, value));
                idx = value_end + 1;
            }
            let _ = tx.send(results);
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

    #[repr(C)]
    union StringUnion<'a> {
        string: &'a str,
        bytes: &'a [u8]
    }

    #[allow(unused)]
    impl<'a> StringUnion<'a> {
        fn test() {
            //                                              67    104   101   110   110   97    105
            let value = StringUnion { bytes: &[0x43, 0x68, 0x65, 0x6E, 0x6E, 0x61, 0x69] };
            println!("as bytes {:?}", unsafe { value.bytes });
            println!("as str {:?}", unsafe { value.string });
        }
    }

}
