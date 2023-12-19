use rodio::{Decoder, OutputStream, source::Source};
use std::{io, thread, time::Duration};
use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor, Color::Rgb},
    terminal::{size, Clear, ClearType, enable_raw_mode, disable_raw_mode},
    event::{poll, read, Event, KeyCode},
    execute,
};
use include_dir::{include_dir, Dir};

static SOUNDS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sounds");

const HIGH_SOUND_FILE: &str = "bright.wav";
const BRIGHT_SOUND_FILE: &str = "high.wav";
const INITIAL_DELAY_TIME: u64 = 60_000;

fn run() -> Result<(), Box<dyn std::error::Error>> {

    clear_screen()?;
    let mut bpm = get_bpm()?;
    enable_raw_mode()?;
    let mut delay_time = INITIAL_DELAY_TIME / (bpm as u64);
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let mut beat_counter = 1;

    loop {
        clear_screen()?;
        display_instructions()?;
        display_bpm(&bpm)?;

        display_beat_symbol(beat_counter)?;

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

fn display_beat_symbol(beat_counter: u32) -> io::Result<()> {
    let beat_symbols = ["\\...", ".|..", "../.", "...-"]; // List of beat symbols
    let symbol = &beat_symbols[(beat_counter - 1) as usize];

    let (cols, _rows) = size().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let pos = (cols / 2).saturating_sub(symbol.len() as u16 / 2);

    execute!(
        io::stdout(),
        MoveTo(pos+1, 6), // Adjust the row as needed
        SetForegroundColor(Rgb { r: 255, g: 101, b: 117 }),
        Print(symbol),
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn get_bpm() -> io::Result<u32> {
    let mut bpm_string = String::new();
    let prompt = "Enter the desired BPM:";
    let (cols, _rows) = size().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let pos = (cols / 2).saturating_sub(prompt.len() as u16 / 2);
    execute!(
        io::stdout(),
        MoveTo(pos, 8),
        SetForegroundColor(Rgb { r: 178, g: 255, b: 209 }),
        Print(prompt),
        SetForegroundColor(Rgb { r: 255, g: 101, b: 117 }),
        MoveTo(pos+10, 10) // Move to the next line
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    io::stdin().read_line(&mut bpm_string)?;
    let bpm: u32 = bpm_string.trim().parse().expect("Please type a number!");
    Ok(bpm)
}

fn clear_screen() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::All))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn get_sound_source(beat_counter: u32) -> Result<Box<dyn Source<Item = f32> + Send>, Box<dyn std::error::Error>> {
    let file = if beat_counter == 1 { 
        SOUNDS_DIR.get_file(HIGH_SOUND_FILE).expect("File not found") 
    } else { 
        SOUNDS_DIR.get_file(BRIGHT_SOUND_FILE).expect("File not found") 
    };
    let buffer = load_sound_file(file)?;
    let source = Decoder::new(io::Cursor::new(buffer))?.convert_samples();
    Ok(Box::new(source))
}

fn load_sound_file(file: &include_dir::File) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(file.contents().to_vec())
}

fn display_bpm(bpm: &u32) -> io::Result<()> {
    let (cols, _rows) = size().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let bpm_display = format!("BPM: {}", bpm);
    let pos = (cols / 2).saturating_sub(bpm_display.len() as u16 / 2);
    execute!(
        io::stdout(),
        // Divider #.1
        MoveTo(pos, 9),
        SetForegroundColor(Rgb { r: 178, g: 255, b: 209 }),
        Print("=========="),
        // BPM
        MoveTo(pos + 1, 10),
        // SetForegroundColor(Rgb { r: 255, g: 101, b: 168 }),
        SetForegroundColor(Rgb { r: 255, g: 101, b: 117 }),
        SetBackgroundColor(Color::Black),
        Print(bpm_display),
        ResetColor,
        // Divider #.2
        MoveTo(pos, 11),
        SetForegroundColor(Rgb { r: 178, g: 255, b: 209 }),
        Print("==========")

    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn display_instructions() -> io::Result<()> {
    let instructions = "(+) to increase bpm, (-) to decrease bpm, (q) to exit";
    let (cols, _rows) = size().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let pos = (cols / 2).saturating_sub(instructions.len() as u16 / 2);
    let mut stdout = io::stdout();

    execute!(stdout, MoveTo(pos, 14))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    for c in instructions.chars() {
        match c {
            '+' | '-' | 'q' => {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Rgb { r: 255, g: 101, b: 117 }), // Same color as BPM
                    Print(c),
                    ResetColor
                )
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            }
            _ => {
                execute!(
                    stdout,
                    SetForegroundColor(Rgb { r: 178, g: 255, b: 209 }),
                    Print(c)
                )
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            }
        }
    }

    Ok(())
}


fn main() {
    match run() {
        Ok(()) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}
