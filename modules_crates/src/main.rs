mod player;

use player::play_movie;
use player::play_audio;
use clean::perform_clean;
use clean::cache::clean_cache;

fn main() {
    play_movie("Creed 3.mp4");
    play_audio("Smack that.mp3");

    perform_clean();
    clean_cache();

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
