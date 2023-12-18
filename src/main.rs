use rodio::{Decoder, OutputStream, source::Source};
use std::{io, thread, time::Duration, fs::File, io::Read};
use crossterm::{
   cursor::{MoveTo},
   style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
   terminal::{size, Clear, ClearType, enable_raw_mode, disable_raw_mode},
   event::{poll, read, Event, KeyCode},
   execute,
};

const HIGH_SOUND_FILE: &str = "sounds/bright.wav";
const BRIGHT_SOUND_FILE: &str = "sounds/high.wav";
const INITIAL_DELAY_TIME: u64 = 60_000;

fn main() -> io::Result<()> {
  let mut bpm = get_bpm();
  enable_raw_mode().map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to enable raw mode"))?;
  let mut delay_time = INITIAL_DELAY_TIME / (bpm as u64);
  let (_stream, stream_handle) = OutputStream::try_default().map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create output stream"))?;

  let mut beat_counter = 1;

  loop {
      clear_screen().map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to clear screen"))?;
      display_bpm(&bpm).map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to display BPM"))?;

      // let source = get_sound_source(buffer_high.clone(), buffer_bright.clone(), beat_counter).map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get sound source"))?;
      let source = get_sound_source(beat_counter).map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get sound source"))?;
      stream_handle.play_raw(source).map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to play sound"))?;

      thread::sleep(Duration::from_millis(delay_time));

      beat_counter = if beat_counter < 4 { beat_counter + 1 } else { 1 };

      if poll(Duration::from_millis(1)).map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to poll event"))? {
          if let Event::Key(event) = read().map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to read event"))? {
              match event.code {
                KeyCode::Char('+') | KeyCode::Up => {
                    bpm += 1;
                    delay_time = 60_000 / (bpm as u64);
                }
                KeyCode::Char('-') | KeyCode::Down => {
                    if bpm > 1 {
                        bpm -= 1;
                        delay_time = 60_000 / (bpm as u64);
                    }
                }
                KeyCode::Char('q') => {
                    disable_raw_mode().map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to disable raw mode"))?;
                    return Ok(());
                }
                _ => {}
              }
          }
      }
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

fn load_sound_file(filename: &str) -> io::Result<Vec<u8>> {
   let mut file = File::open(filename)?;
   let mut buffer = Vec::new();
   file.read_to_end(&mut buffer)?;
   Ok(buffer)
}

fn clear_screen() -> io::Result<()> {
  execute!(io::stdout(), Clear(ClearType::All)).map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to clear screen"))
}

fn display_bpm(bpm: &u32) -> io::Result<()> {
  let (cols, _rows) = size().map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get terminal size"))?;
  let pos = (cols / 2) as u16;

  execute!(
      io::stdout(),
      MoveTo(pos, 10),
      SetForegroundColor(Color::Red),
      SetBackgroundColor(Color::Black),
      Print(format!("BPM: {}", bpm)),
      ResetColor
  ).map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to display BPM"))
}

fn create_decoder(filename: &str) -> io::Result<Box<dyn Source<Item = f32> + Send>> {
   let buffer = load_sound_file(filename)?;
   let source = Decoder::new(io::Cursor::new(buffer)).map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create decoder"))?.convert_samples();
   Ok(Box::new(source))
}


fn get_sound_source(beat_counter: u32) -> io::Result<Box<dyn Source<Item = f32> + Send>> {
   let source = if beat_counter == 1 {
       create_decoder(HIGH_SOUND_FILE)
   } else {
       create_decoder(BRIGHT_SOUND_FILE)
   };
   source
}
