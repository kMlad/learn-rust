mod player;
mod archive;

use player::play_movie;
use player::play_audio;
use clean::perform_clean;
use clean::cache::clean_cache;
use crate::archive::arch::arch_file as arc;
use rand::Rng;

fn main() {
    play_movie("Creed 3.mp4");
    play_audio("Smack that.mp3");

    println!("--------------------------");

    perform_clean();
    clean_cache();

    println!("--------------------------");


    arc("hehence.rs");

    println!("--------------------------");


    let mut rng = rand::thread_rng();
    let generated_number: u8 = rng.gen_range(1..100);
    println!("Your number is {:?}", generated_number);

}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning HDD...")
    }

    pub mod cache {
        pub fn clean_cache() {
            println!("Cleaning unused cache...")
        }
    }
}
