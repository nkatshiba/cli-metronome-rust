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
    let mut file = File::open("sounds/high.wav").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // Infinite loop to simulate the metronome
    loop {
        // Play the sound by creating a new Decoder each time
        let cursor = io::Cursor::new(buffer.clone());
        let source = Decoder::new(cursor).unwrap().convert_samples();
        stream_handle.play_raw(source).unwrap();

        // Sleep for the calculated delay time
        thread::sleep(Duration::from_millis(delay_time as u64));
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
