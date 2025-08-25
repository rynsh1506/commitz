use std::io::{self, stdout, Write};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
};

pub fn move_down(
    selected: &mut usize,
    cursor: &mut usize,
    offset: &mut usize,
    len: usize,
    window_size: usize,
) {
    *selected = (*selected + 1) % len;

    let stick_pos = window_size / 2;

    if *cursor < stick_pos {
        *cursor += 1;
    } else {
        *offset = (*offset + 1) % len;
    }
}

pub fn move_up(
    selected: &mut usize,
    cursor: &mut usize,
    offset: &mut usize,
    len: usize,
    window_size: usize,
) {
    let stick_pos = window_size / 2;

    *selected = (*selected + len - 1) % len;

    if *cursor > 0 && *cursor > stick_pos {
        *cursor -= 1;
    } else {
        *offset = (*offset + len - 1) % len;
    }
}

pub fn handle_input(
    selected: &mut usize,
    cursor: &mut usize,
    offset: &mut usize,
    len: usize,
    window_size: usize,
) -> io::Result<Option<usize>> {
    if let Event::Key(event) = event::read()? {
        match event.code {
            KeyCode::Down => move_down(selected, cursor, offset, len, window_size),
            KeyCode::Up => move_up(selected, cursor, offset, len, window_size),
            KeyCode::Enter => return Ok(Some(*selected)),
            KeyCode::Esc => {}
            KeyCode::Char('c') if event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                terminal::disable_raw_mode()?;
                execute!(
                    stdout(),
                    terminal::LeaveAlternateScreen,
                    SetForegroundColor(Color::Red),
                    Print("\n❌ Aborted by user (Ctrl+C).\n"),
                    ResetColor
                )?;
                std::process::exit(1);
            }
            _ => {}
        }
    }
    Ok(None)
}

pub fn handle_prompt_input() -> io::Result<String> {
    let mut buffer = String::new();
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    loop {
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char('c') if event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                    terminal::disable_raw_mode()?;
                    execute!(
                        stdout,
                        terminal::LeaveAlternateScreen,
                        SetForegroundColor(Color::Red),
                        Print("\n❌ Aborted by user (Ctrl+C).\n"),
                        ResetColor
                    )?;
                    std::process::exit(1);
                }
                KeyCode::Char(c) => {
                    buffer.push(c);
                    write!(stdout, "{}", c)?;
                    stdout.flush()?;
                }
                KeyCode::Backspace => {
                    if buffer.pop().is_some() {
                        write!(stdout, "\x08 \x08")?;
                        stdout.flush()?;
                    }
                }
                KeyCode::Enter => {
                    write!(stdout, "")?;
                    stdout.flush()?;
                    break;
                }
                KeyCode::Esc => {
                    buffer.clear();
                    break;
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(buffer)
}
