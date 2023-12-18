use rodio::{Decoder, OutputStream, source::Source};
use std::{io, thread, time::Duration, fs::File, io::Read};

fn main() {
    // Get BPM from user
    let bpm = get_bpm();

    // Calculate the delay time in milliseconds
    let delay_time = 60_000 / bpm;

    // Initialize audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load the "high.wav" sound file into memory
    let mut file_high = File::open("sounds/bright.wav").unwrap();
    let mut buffer_high = Vec::new();
    file_high.read_to_end(&mut buffer_high).unwrap();

    // Load the "bright.wav" sound file into memory
    let mut file_bright = File::open("sounds/high.wav").unwrap();
    let mut buffer_bright = Vec::new();
    file_bright.read_to_end(&mut buffer_bright).unwrap();

    // Initialize beat counter
    let mut beat_counter = 1;

    // Infinite loop to simulate the metronome
    loop {
        // Play the appropriate sound by creating a new Decoder each time
        let cursor = if beat_counter == 1 {
            io::Cursor::new(buffer_high.clone())
        } else {
            io::Cursor::new(buffer_bright.clone())
        };
        let source = Decoder::new(cursor).unwrap().convert_samples();
        stream_handle.play_raw(source).unwrap();

        // Sleep for the calculated delay time
        thread::sleep(Duration::from_millis(delay_time as u64));

        // Update beat counter
        beat_counter = if beat_counter < 4 { beat_counter + 1 } else { 1 };
    }
}

fn get_bpm() -> u32 {
    let mut bpm_string = String::new();
    println!("Enter the desired BPM:");

    io::stdin()
        .read_line(&mut bpm_string)
        .expect("Failed to read line");

    let bpm: u32 = bpm_string.trim().parse().expect("Please type a number!");

    bpm
}
