use std::thread;
use vlc::{Instance, Media, MediaPlayer};

fn main() {
    println!("Hello, world!");
    let instance = Instance::new().unwrap();
    // Create a media from a file
    let md = Media::new_path(&instance, "file_example_MP3_700KB.mp3").unwrap();
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait for 10 seconds
    thread::sleep(::std::time::Duration::from_secs(10));
}
