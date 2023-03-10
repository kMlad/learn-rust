extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};


fn main() {
    let out = b"Ne ide zivot, al ide gas!";
    let width = 14;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}
