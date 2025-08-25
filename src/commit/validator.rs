use std::{
    io::{self, stdout, Write},
    process::Command,
};

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};

use crate::commit::navigation::handle_prompt_input;

pub fn confirm_question(prompt: &str) -> io::Result<bool> {
    let mut stdout = stdout();
    let (_, term_height) = terminal::size()?;
    let prompt_row = term_height - 2;

    loop {
        execute!(
            stdout,
            cursor::MoveTo(0, prompt_row),
            terminal::Clear(ClearType::CurrentLine),
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
        stdout.flush()?;

        let input = handle_prompt_input()?.trim().to_lowercase();

        match input.as_str() {
            "y" | "yes" => {
                execute!(stdout, cursor::RestorePosition)?;
                stdout.flush()?;
                return Ok(true);
            }
            "n" | "no" => {
                execute!(
                    stdout,
                    cursor::MoveTo(0, prompt_row + 1),
                    terminal::Clear(ClearType::CurrentLine),
                    SetForegroundColor(Color::Red),
                    Print("❌ Commit canceled by user.\n"),
                    ResetColor
                )?;
                return Ok(false);
            }
            _ => {
                execute!(
                    stdout,
                    cursor::MoveTo(0, prompt_row + 1),
                    terminal::Clear(ClearType::CurrentLine),
                    SetForegroundColor(Color::Red),
                    Print("Invalid input! Please enter Y or N."),
                    ResetColor,
                    cursor::MoveTo(2 + prompt.len() as u16, prompt_row)
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
