use onebrc_lib::{init_mmap, make_me_the_good_good};

fn main() {
    let input_filename = std::env::args().nth(1).expect("No input filename");
    init_mmap(Some(&input_filename));
    // init_mmap(Some("../measurements-1_000_000.txt"));
    make_me_the_good_good(false);
}
