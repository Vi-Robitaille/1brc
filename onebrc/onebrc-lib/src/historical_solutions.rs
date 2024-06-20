

#[allow(unused)]
pub fn doodoofard() {
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