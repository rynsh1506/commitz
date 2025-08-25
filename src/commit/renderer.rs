use std::io::{self, stdout, Write};

use crossterm::{
    cursor::{self},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};

use crate::commit::{
    navigation::handle_input,
    types::{CommitType, RenderCommit},
};

pub fn render_commit(stdout: &mut impl Write, parts: &RenderCommit) -> io::Result<()> {
    let (open_paren, inner, close_paren) =
        if parts.final_scope.starts_with('(') && parts.final_scope.ends_with(')') {
            ("(", &parts.final_scope[1..parts.final_scope.len() - 1], ")")
        } else {
            ("", &parts.final_scope[..], "")
        };

    execute!(
        stdout,
        Print("\n\n"),
        SetForegroundColor(Color::DarkGreen),
        Print("✔ Generated Git Commit Message:\n"),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print("###--------------------------------------------------------###\n"),
        ResetColor,
        SetForegroundColor(Color::DarkGreen),
        Print(&parts.chosen_type),
        ResetColor,
        Print(open_paren),
        SetForegroundColor(Color::Yellow),
        Print(inner),
        ResetColor,
        Print(close_paren),
        ResetColor,
        Print(format!(": {}", &parts.desc))
    )?;

    if !parts.longer_description.is_empty() {
        execute!(stdout, Print(format!("\n\n{}", &parts.longer_description)))?;
    }
    if !parts.breaking_changes.is_empty() {
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print(format!("\n\nBREAKING CHANGE: {}", &parts.breaking_changes)),
            ResetColor
        )?;
    }
    if !parts.issue_refs.is_empty() {
        execute!(
            stdout,
            Print(format!("\n\n{} {}", &parts.issue_prefix, &parts.issue_refs)),
            ResetColor
        )?;
    }

    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("\n###--------------------------------------------------------###\n\n"),
        ResetColor,
        cursor::SavePosition,
    )?;

    Ok(())
}

pub fn render_footer(stdout: &mut impl Write, row: u16) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, row),
        SetForegroundColor(Color::DarkGrey),
        Print("↑/↓ to move, Enter to select, Ctrl-C to cancel"),
        ResetColor
    )
}

pub fn render_options(
    stdout: &mut impl Write,
    types: &[CommitType],
    selected: usize,
    offset: usize,
    window_size: usize,
) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 2),
        terminal::Clear(ClearType::FromCursorDown)
    )?;

    let max_key_len = types.iter().map(|t| t.key.len()).max().unwrap_or(0);
    let len = types.len();

    for i in 0..window_size {
        let idx = if len <= window_size {
            i
        } else {
            (offset + i) % len
        };

        let t = &types[idx];
        let row = (i + 2) as u16;

        let line = if t.description.is_empty() {
            format!("{:<width$}", t.key, width = max_key_len)
        } else {
            format!("{:<width$} : {}", t.key, t.description, width = max_key_len)
        };

        if idx == selected {
            execute!(
                stdout,
                cursor::MoveTo(0, row),
                SetForegroundColor(Color::Yellow),
                Print("❯ "),
                SetForegroundColor(Color::DarkGreen),
                Print(line),
                ResetColor
            )?;
        } else {
            execute!(
                stdout,
                cursor::MoveTo(0, row),
                Print(format!("  {}", line)),
                ResetColor
            )?;
        }
    }

    render_footer(stdout, (window_size + 3) as u16)?;
    stdout.flush()
}

pub fn render_scope() -> io::Result<String> {
    let mut stdout = stdout();
    let scopes = ["empty", "custom"];
    let mut selected = 0;
    let mut offset = 0;
    let mut cursor = 0;
    let window_size = 2;

    stdout.flush()?;

    let chosen_scope = loop {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGreen),
            cursor::MoveTo(0, 1),
            Print("? "),
            ResetColor,
            Print("Denote the SCOPE of this change (optional):"),
            ResetColor,
        )?;

        render_options(
            &mut stdout,
            &scopes
                .iter()
                .map(|s| CommitType::new(s, ""))
                .collect::<Vec<_>>(),
            selected,
            offset,
            window_size,
        )?;

        if let Some(chosen) = handle_input(
            &mut selected,
            &mut cursor,
            &mut offset,
            scopes.len(),
            window_size,
        )? {
            terminal::disable_raw_mode()?;
            execute!(
                stdout,
                terminal::Clear(ClearType::CurrentLine),
                cursor::RestorePosition,
                cursor::MoveTo(0, 1),
                Print("? "),
                ResetColor,
                Print("Denote the SCOPE of this change (optional):"),
                SetForegroundColor(Color::DarkGreen),
                Print(format!(" {}\n", scopes[chosen])),
                ResetColor,
            )?;
            break scopes[chosen].to_string();
        }
    };

    Ok(chosen_scope)
}
