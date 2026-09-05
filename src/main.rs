use std::{
    env,
    fs,
    io::{self, Write},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};

use colored::{
    Colorize
};

fn main() -> io::Result<()> {
    let current_directory = env::current_dir()?;
    let mut entries = Vec::new();

    for entry in fs::read_dir(&current_directory)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        entries.push((
            entry.file_name().to_string_lossy().to_string(),
            file_type.is_dir(),
        ));
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut selected: usize = 0;

    terminal::enable_raw_mode()?;

    let result = run(&entries, &mut selected);

    terminal::disable_raw_mode()?;

    result
}

fn run(entries: &[(String, bool)],selected: &mut usize,) -> io::Result<()> {
    loop {
        let mut stdout = io::stdout();

        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        for (index, (name, is_directory)) in entries.iter().enumerate() {
            let prefix = if index == *selected { ">  ".green().bold() } else { "  ".normal() };
            let suffix = if *is_directory { "/" } else { "" };

            let display_name = if *is_directory { 
                name.blue().bold()
            } else {
                name.white()
            };

            write!(stdout, "{prefix}{display_name}{suffix}\r\n");
        }

        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    *selected = selected.saturating_sub(1);
                }

                KeyCode::Down => {
                    if *selected + 1 < entries.len() {
                        *selected += 1;
                    }
                }

                KeyCode::Char('q') | KeyCode::Esc => {
                    break;
                }

                _ => {}
            }
        }
    }

    Ok(())
}
