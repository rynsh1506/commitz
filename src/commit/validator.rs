use std::{
    io::{self, stdout, Write},
    process::Command,
};

use crossterm::{
    cursor::{self, RestorePosition, SavePosition},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};

use crate::commit::navigation::handle_prompt_input;

pub fn confirm_question(prompt: &str) -> io::Result<bool> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    let (_, row) = cursor::position()?;
    execute!(stdout, SavePosition)?;

    execute!(
        stdout,
        cursor::MoveTo(0, row - 1),
        SetForegroundColor(Color::DarkGreen),
        Print("? "),
        ResetColor,
        Print(prompt),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(" (Y/n) "),
        ResetColor,
        SetForegroundColor(Color::DarkGreen),
        SavePosition
    )?;

    loop {
        execute!(
            stdout,
            RestorePosition,
            terminal::Clear(ClearType::UntilNewLine)
        )?;
        stdout.flush()?;

        let input = handle_prompt_input()?;

        let input_lower = input.trim().to_lowercase();

        match input_lower.as_str() {
            "y" | "yes" => {
                terminal::disable_raw_mode()?;
                return Ok(true);
            }
            "n" | "no" => {
                execute!(
                    stdout,
                    RestorePosition,
                    terminal::Clear(ClearType::UntilNewLine),
                    cursor::MoveToNextLine(1),
                    SetForegroundColor(Color::Red),
                    Print("❌ Commit canceled by user.\n"),
                    ResetColor
                )?;
                terminal::disable_raw_mode()?;
                return Ok(false);
            }
            _ => {
                execute!(
                    stdout,
                    terminal::Clear(ClearType::CurrentLine),
                    cursor::MoveTo(0, row - 1),
                    SetForegroundColor(Color::DarkGreen),
                    Print("? "),
                    ResetColor,
                    Print(prompt),
                    ResetColor,
                    SetForegroundColor(Color::DarkGrey),
                    Print(" (Y/n) "),
                    ResetColor,
                    SetForegroundColor(Color::DarkGreen),
                )?;
                execute!(
                    stdout,
                    cursor::MoveTo(0, row),
                    SetForegroundColor(Color::Red),
                    Print("Invalid input! Please enter Y or N."),
                    ResetColor,
                )?;
                stdout.flush()?;
            }
        }
    }
}

pub fn ensure_staged_files() -> io::Result<()> {
    let status = Command::new("git")
        .args([
            "diff",
            "--cached",
            "--no-ext-diff",
            "--name-only",
            "--quiet",
        ])
        .status();

    match status {
        Ok(s) if !s.success() => Ok(()),
        Ok(_) => {
            execute!(
                stdout(),
                SetForegroundColor(Color::Yellow),
                Print("No files added to staging! Did you forget to run `git add`?\n"),
                ResetColor
            )?;
            std::process::exit(1);
        }
        Err(_) => {
            execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print("Failed to run git command.\n"),
                ResetColor
            )?;
            std::process::exit(1);
        }
    }
}
