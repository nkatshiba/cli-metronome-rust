use rodio::{Decoder, OutputStream, source::Source};
use std::{io, thread, time::Duration, fs::File, io::Read};
use crossterm::{
    cursor::{MoveTo, position},
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{size, Clear, ClearType, enable_raw_mode, disable_raw_mode},
    event::{poll, read, Event, KeyCode},
    execute,
};

fn main() {

    // Get BPM from user
    let mut bpm = get_bpm();

    // Enable raw mode to read user input without pressing enter
    enable_raw_mode().unwrap();

    // Calculate the delay time in milliseconds
    let mut delay_time = 60_000 / bpm;

    // Initialize audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load the "high.wav" sound file into memory
    // let mut file_high = File::open("sounds/bright.wav").unwrap();
    // let mut buffer_high = Vec::new();
    // file_high.read_to_end(&mut buffer_high).unwrap();
    //
    // // Load the "bright.wav" sound file into memory
    // let mut file_bright = File::open("sounds/high.wav").unwrap();
    // let mut buffer_bright = Vec::new();
    // file_bright.read_to_end(&mut buffer_bright).unwrap();

    let buffer_high = load_sound_file("sounds/bright.wav").unwrap();
    let buffer_bright = load_sound_file("sounds/high.wav").unwrap();

    // Initialize beat counter
    let mut beat_counter = 1;

    loop {
      // ...

      // Clear the screen
      execute!(io::stdout(), Clear(ClearType::All)).unwrap();

      // Get the size of the terminal
      let (cols, _rows) = size().unwrap();

      // Calculate the position to center the BPM text
      let pos = (cols / 2) as u16;

      // Display the BPM in the middle of the screen in big text
      execute!(
          io::stdout(),
          MoveTo(pos, 10), // Move cursor to the middle of the screen
          SetForegroundColor(Color::Red), // Set text color
          SetBackgroundColor(Color::Black), // Set background color
          Print(format!("BPM: {}", bpm)), // Print the BPM
          ResetColor // Reset color to default
      ).unwrap();

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

      // Check if there's any user input
      if poll(Duration::from_millis(1)).unwrap() {
          if let Event::Key(event) = read().unwrap() {
              match event.code {
                  // Increase BPM when '+' or 'up' is pressed
                  KeyCode::Char('+') | KeyCode::Up => {
                      bpm += 1;
                      delay_time = 60_000 / bpm;
                  }
                  // Decrease BPM when '-' or 'down' is pressed
                  KeyCode::Char('-') | KeyCode::Down => {
                      if bpm > 1 {
                          bpm -= 1;
                          delay_time = 60_000 / bpm;
                      }
                  }
                  // Quit when 'q' is pressed
                  KeyCode::Char('q') => {
                      disable_raw_mode().unwrap();
                      return;
                  }
                  _ => {}
              }
          }
      }
    }

    // Infinite loop to simulate the metronome
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


fn load_sound_file(filename: &str) -> io::Result<Vec<u8>> {
   let mut file = File::open(filename)?;
   let mut buffer = Vec::new();
   file.read_to_end(&mut buffer)?;
   Ok(buffer)
}
