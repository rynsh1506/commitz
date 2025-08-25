use std::io::{self, stdout, Write};

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};

use crate::commit::{
    navigation::{handle_input, handle_prompt_input},
    renderer::render_options,
    types::CommitType,
};

pub fn read_desc() -> io::Result<String> {
    let mut stdout = stdout();
    let mut error_printed = false;
    let mut input = String::new();

    execute!(
        stdout,
        SetForegroundColor(Color::DarkGreen),
        Print("? "),
        ResetColor,
        Print("Write a SHORT, IMPERATIVE tense description of the change:"),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print("\n[Infinity more chars allowed]\n "),
        ResetColor,
        SetForegroundColor(Color::DarkGreen),
        cursor::SavePosition,
    )?;
    stdout.flush()?;

    loop {
        input.clear();
        input = handle_prompt_input()?;
        let trimmed = input.trim();

        if !trimmed.is_empty() {
            execute!(stdout, ResetColor)?;
            return Ok(trimmed.to_string());
        } else if !error_printed {
            execute!(
                stdout,
                cursor::RestorePosition,
                terminal::Clear(ClearType::CurrentLine),
                cursor::MoveToColumn(0),
                SetForegroundColor(Color::Red),
                Print("\n>> [ERROR] input is required"),
                ResetColor,
                cursor::RestorePosition,
            )?;
            stdout.flush()?;
            error_printed = true;
        } else {
            execute!(stdout, cursor::RestorePosition)?;
            stdout.flush()?;
        }
    }
}
pub fn read_multiline(prompt: &str) -> io::Result<String> {
    let mut stdout = stdout();
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGreen),
        Print("\n? "),
        ResetColor,
        Print(prompt),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print("(press Enter to skip):\n"),
        ResetColor,
        SetForegroundColor(Color::DarkGreen)
    )?;
    stdout.flush()?;

    let input = handle_prompt_input()?;

    if input.is_empty() {
        execute!(
            stdout,
            terminal::Clear(ClearType::CurrentLine),
            cursor::RestorePosition
        )?
    };

    Ok(input
        .trim()
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n"))
}

pub fn read_issues() -> io::Result<(String, String)> {
    let mut stdout = stdout();
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGreen),
        Print("\n? "),
        ResetColor,
        Print("Select the ISSUES type of change (optional), Input ISSUES prefix\n"),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print("(press Enter to skip):\n"),
        ResetColor,
    )?;
    io::stdout().flush()?;

    execute!(stdout, SetForegroundColor(Color::DarkGreen))?;
    stdout.flush()?;

    let issue_prefix = handle_prompt_input()?;

    let mut issue_refs = String::new();

    if !issue_prefix.is_empty() {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGreen),
            Print("\n? "),
            ResetColor,
            Print("List any ISSUES AFFECTED by this change. E.g.: #31, #34: "),
            ResetColor,
        )?;
        io::stdout().flush()?;

        execute!(stdout, SetForegroundColor(Color::DarkGreen))?;
        stdout.flush()?;

        issue_refs = handle_prompt_input()?.trim().to_string();
    } else {
        execute!(
            stdout,
            terminal::Clear(ClearType::CurrentLine),
            cursor::RestorePosition
        )?
    }

    Ok((issue_prefix, issue_refs))
}
pub fn read_commit_type() -> io::Result<CommitType> {
    let mut stdout = stdout();
    let types = CommitType::load();
    let mut selected = 0;
    let mut offset = 0;
    let mut cursor = 0;
    let window_size = 7;

    let chosen_type = loop {
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::DarkGreen),
            Print("? "),
            ResetColor,
            Print("Select the type of change that you're committing: "),
            ResetColor
        )?;
        render_options(&mut stdout, &types, selected, offset, window_size)?;
        if let Some(chosen) = handle_input(
            &mut selected,
            &mut cursor,
            &mut offset,
            types.len(),
            window_size,
        )? {
            break &types[chosen];
        }
    };

    Ok(CommitType::new(&chosen_type.key, &chosen_type.description))
}
pub fn read_scope(chosen_scope: &str) -> io::Result<String> {
    let mut stdout = stdout();
    let final_scope = if chosen_scope == "custom" {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGreen),
            Print("? "),
            ResetColor,
            Print("Denote the SCOPE of this change: "),
            ResetColor,
            SetForegroundColor(Color::DarkGreen),
        )?;
        stdout.flush()?;
        let custom_scope = handle_prompt_input()?;
        format!("({})", custom_scope.trim())
    } else {
        "".to_string()
    };

    Ok(final_scope)
}
