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

fn main() {
    match run() {
        Ok(()) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut bpm = get_bpm()?;
    enable_raw_mode()?;
    let mut delay_time = INITIAL_DELAY_TIME / (bpm as u64);
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let mut beat_counter = 1;

    loop {
        clear_screen()?;
        display_bpm(&bpm)?;
        let source = get_sound_source(beat_counter)?;
        stream_handle.play_raw(source.convert_samples())?;
        thread::sleep(Duration::from_millis(delay_time));
        beat_counter = if beat_counter < 4 { beat_counter + 1 } else { 1 };

        if poll(Duration::from_millis(1))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Char('+') | KeyCode::Up => {
                        bpm += 1;
                        delay_time = INITIAL_DELAY_TIME / (bpm as u64);
                    },
                    KeyCode::Char('-') | KeyCode::Down => {
                        if bpm > 1 {
                            bpm -= 1;
                            delay_time = INITIAL_DELAY_TIME / (bpm as u64);
                        }
                    },
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        return Ok(());
                    },
                    _ => {}
                }
            }
        }
    }
}

fn get_bpm() -> io::Result<u32> {
    let mut bpm_string = String::new();
    println!("Enter the desired BPM:");
    io::stdin().read_line(&mut bpm_string)?;
    let bpm: u32 = bpm_string.trim().parse().expect("Please type a number!");
    Ok(bpm)
}

fn clear_screen() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::All))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn display_bpm(bpm: &u32) -> io::Result<()> {
    let (cols, _rows) = size().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let pos = (cols / 2) as u16;
    execute!(
        io::stdout(),
        MoveTo(pos, 10),
        SetForegroundColor(Color::Red),
        SetBackgroundColor(Color::Black),
        Print(format!("BPM: {}", bpm)),
        ResetColor
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn get_sound_source(beat_counter: u32) -> Result<Box<dyn Source<Item = f32> + Send>, Box<dyn std::error::Error>> {
    let filename = if beat_counter == 1 { HIGH_SOUND_FILE } else { BRIGHT_SOUND_FILE };
    let buffer = load_sound_file(filename)?;
    let source = Decoder::new(io::Cursor::new(buffer))?.convert_samples();
    Ok(Box::new(source))
}

fn load_sound_file(filename: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
