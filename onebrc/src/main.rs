use onebrc_lib::{init_mmap, statemachine};

fn main() {
    init_mmap(None);
    statemachine::make_me_the_good_good(true);
}
