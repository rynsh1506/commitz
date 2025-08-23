use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType, EnterAlternateScreen},
};
use serde::Deserialize;
use std::io;
use std::{
    io::{stdout, Write},
    process::Command,
};

const COMMIT_TYPES_JSON: &str = r#"
{
  "types": [
    { "key": "feat", "description": "A new feature" },
    { "key": "fix", "description": "A bug fix" },
    { "key": "doc", "description": "Documentation only changes" },
    { "key": "style", "description": "Changes that do not affect the meaning of the code" },
    { "key": "refactor", "description": "A code change that neither fixes a bug nor adds a feature" },
    { "key": "pref", "description": "A code change that improves performance" },
    { "key": "test", "description": "Adding missing tests or correcting existing tests" },
    { "key": "ci", "description": "Continuous Integration related changes" },
    { "key": "chore", "description": "Other changes that do not modify src or test files" }
  ]
}
"#;
#[derive(Debug, Deserialize)]
struct CommitType {
    key: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct CommitTypes {
    types: Vec<CommitType>,
}

impl CommitType {
    fn load() -> Vec<CommitType> {
        let commit_types: CommitTypes =
            serde_json::from_str(COMMIT_TYPES_JSON).expect("Invalid embedded JSON format");
        commit_types.types
    }
}

fn render_header(stdout: &mut impl Write) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        SetForegroundColor(Color::DarkGreen),
        Print("? "),
        ResetColor,
        SetForegroundColor(Color::White),
        Print("Select the type of change that you're committing: "),
        SetForegroundColor(Color::DarkGrey),
        Print("Use arrow keys to move, Enter to select\n\n"),
        ResetColor
    )
}

fn render_footer(stdout: &mut impl Write, row: u16) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, row),
        SetForegroundColor(Color::DarkGrey),
        Print("↑/↓ to move, Enter to select, Esc to cancel"),
        ResetColor
    )
}

fn render_options(
    stdout: &mut impl Write,
    types: &[CommitType],
    selected: usize,
    offset: usize,
    window_size: usize,
    max_key_len: usize,
) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 2),
        terminal::Clear(ClearType::FromCursorDown)
    )?;

    for i in 0..window_size {
        let idx = offset + i;
        if idx >= types.len() {
            break;
        }
        let t = &types[idx];
        let row = (i + 2) as u16;

        if idx == selected {
            execute!(
                stdout,
                cursor::MoveTo(0, row),
                SetForegroundColor(Color::Yellow),
                Print("❯ "),
                SetForegroundColor(Color::DarkGreen),
                Print(format!(
                    "{:<width$} : {}",
                    t.key,
                    t.description,
                    width = max_key_len
                )),
                ResetColor
            )?;
        } else {
            execute!(
                stdout,
                cursor::MoveTo(0, row),
                SetForegroundColor(Color::White),
                Print(format!(
                    "  {:<width$} : {}",
                    t.key,
                    t.description,
                    width = max_key_len
                )),
                ResetColor
            )?;
        }
    }

    render_footer(stdout, (window_size + 3) as u16)?;
    stdout.flush()
}

fn move_down(selected: &mut usize, offset: &mut usize, len: usize, window_size: usize) {
    if *selected + 1 >= len {
        *selected = 0;
        *offset = 0;
    } else {
        *selected += 1;
        if *selected >= *offset + window_size {
            *offset += 1;
        }
    }
}

fn move_up(selected: &mut usize, offset: &mut usize, len: usize, window_size: usize) {
    if *selected == 0 {
        *selected = len - 1;
        *offset = len.saturating_sub(window_size);
    } else {
        *selected -= 1;
        if *selected < *offset {
            *offset = *selected;
        }
    }
}

fn handle_input(
    selected: &mut usize,
    offset: &mut usize,
    len: usize,
    window_size: usize,
) -> io::Result<Option<usize>> {
    if let Event::Key(event) = event::read()? {
        match event.code {
            KeyCode::Down => move_down(selected, offset, len, window_size),
            KeyCode::Up => move_up(selected, offset, len, window_size),
            KeyCode::Enter => return Ok(Some(*selected)),
            KeyCode::Esc => {
                // Keluarkan dari program langsung
                terminal::disable_raw_mode()?; // jangan lupa matikan raw mode dulu
                std::process::exit(0);
            }
            _ => {}
        }
    }
    Ok(None)
}

fn read_desc(prompt: &str) -> String {
    let mut stdout = stdout();
    let mut input = String::new();
    let mut error_printed = false;

    execute!(
        stdout,
        SetForegroundColor(Color::DarkGreen),
        Print("? "),
        ResetColor,
        SetForegroundColor(Color::White),
        Print(prompt),
        ResetColor,
        Print("\n[Infinity more chars allowed]\n "),
        ResetColor,
        cursor::SavePosition
    )
    .unwrap();
    stdout.flush().unwrap();

    execute!(stdout, SetForegroundColor(Color::DarkGreen)).unwrap();
    stdout.flush().unwrap();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if !trimmed.is_empty() {
            execute!(stdout, ResetColor).unwrap();
            return trimmed.to_string();
        } else if !error_printed {
            execute!(
                stdout,
                cursor::RestorePosition,
                terminal::Clear(ClearType::CurrentLine),
                cursor::MoveToColumn(0),
                SetForegroundColor(Color::Red),
                Print("\n>> [ERROR] input is required"),
                ResetColor,
                cursor::RestorePosition
            )
            .unwrap();
            stdout.flush().unwrap();
            error_printed = true;
        } else {
            execute!(stdout, cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
        }
    }
}

fn read_multiline(prompt: &str) -> String {
    let mut stdout = stdout();
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGreen),
        Print("? "),
        ResetColor,
        SetForegroundColor(Color::White),
        Print(prompt),
        ResetColor,
        SetForegroundColor(Color::DarkGreen),
        Print(" "),
        ResetColor,
        cursor::SavePosition
    )
    .unwrap();
    stdout.flush().unwrap();

    execute!(stdout, SetForegroundColor(Color::DarkGreen)).unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
        .trim()
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn read_issues() -> (String, String) {
    let mut stdout = stdout();

    // Tanya jenis issue
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGreen),
        Print("? "),
        ResetColor,
        SetForegroundColor(Color::White),
        Print("Select the ISSUES type of change (optional), Input ISSUES prefix: "),
        ResetColor,
    )
    .unwrap();
    io::stdout().flush().unwrap();

    execute!(stdout, SetForegroundColor(Color::DarkGreen)).unwrap();
    stdout.flush().unwrap();

    let mut issue_prefix = String::new();
    io::stdin().read_line(&mut issue_prefix).unwrap();
    let issue_prefix = issue_prefix.trim().to_string();

    let mut issue_refs = String::new();

    if !issue_prefix.is_empty() {
        // Tanya daftar issues
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGreen),
            Print("? "),
            ResetColor,
            SetForegroundColor(Color::White),
            Print("List any ISSUES AFFECTED by this change. E.g.: #31, #34: "),
            ResetColor,
        )
        .unwrap();
        io::stdout().flush().unwrap();

        execute!(stdout, SetForegroundColor(Color::DarkGreen)).unwrap();
        stdout.flush().unwrap();

        io::stdin().read_line(&mut issue_refs).unwrap();
        issue_refs = issue_refs.trim().to_string();
    }

    (issue_prefix, issue_refs)
}

fn ensure_git_repo() -> io::Result<()> {
    let status = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => {
            execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print("Not a git repository! Run `git init` first.\n"),
                ResetColor
            )?;
            std::process::exit(1);
        }
    }
}

fn ensure_staged_files() -> io::Result<()> {
    let status = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .status();

    match status {
        Ok(s) if !s.success() => Ok(()), // ada file yang di-staged
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
fn main() -> io::Result<()> {
    ensure_git_repo()?;
    ensure_staged_files()?;

    let types = CommitType::load();
    let max_key_len = types.iter().map(|t| t.key.len()).max().unwrap_or(0);

    let mut selected = 0;
    let mut offset = 0;
    let window_size = 5;

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    // Masuk ke alternate screen
    execute!(stdout, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    // Pilih commit type
    render_header(&mut stdout)?;
    let chosen_type = loop {
        render_options(
            &mut stdout,
            &types,
            selected,
            offset,
            window_size,
            max_key_len,
        )?;
        if let Some(chosen) = handle_input(&mut selected, &mut offset, types.len(), window_size)? {
            break &types[chosen];
        }
    };

    // Pilih scope
    let scopes = ["empty", "custom"];
    let mut scope_selected = 0;

    let chosen_scope = loop {
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::DarkGreen),
            Print("? "),
            ResetColor,
            SetForegroundColor(Color::White),
            Print(format!(
                "Denote the SCOPE of this change (optional) [{}]:",
                chosen_type.key
            )),
            ResetColor
        )?;

        render_options(
            &mut stdout,
            &scopes
                .iter()
                .map(|s| CommitType {
                    key: s.to_string(),
                    description: "".to_string(),
                })
                .collect::<Vec<_>>(),
            scope_selected,
            0,
            scopes.len(),
            6,
        )?;

        if let Some(chosen) = handle_input(&mut scope_selected, &mut 0, scopes.len(), scopes.len())?
        {
            terminal::disable_raw_mode()?;
            break scopes[chosen];
        }
    };

    // Tampilkan pilihan awal
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        SetForegroundColor(Color::DarkGreen),
        Print("? "),
        ResetColor,
        SetForegroundColor(Color::White),
        Print("Select the type of change that you're committing: "),
        ResetColor,
        SetForegroundColor(Color::DarkGreen),
        Print(format!("{}: {}", chosen_type.key, chosen_type.description)),
        ResetColor,
    )?;

    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGreen),
        Print("? "),
        ResetColor,
        SetForegroundColor(Color::White),
        Print("Denote the SCOPE of this change (optional): "),
        ResetColor,
        SetForegroundColor(Color::DarkGreen),
        Print(chosen_scope),
    )?;
    println!();

    // Kalau custom → tanya scope
    let final_scope = if chosen_scope == "custom" {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGreen),
            Print("? "),
            ResetColor,
            SetForegroundColor(Color::White),
            Print("Denote the SCOPE of this change: "),
            ResetColor,
            SetForegroundColor(Color::DarkGreen),
        )?;
        io::stdout().flush()?;
        let mut custom_scope = String::new();
        io::stdin().read_line(&mut custom_scope)?;
        custom_scope.trim().to_string()
    } else {
        chosen_scope.to_string()
    };

    // Lanjut isi deskripsi
    let desc = read_desc("Write a SHORT, IMPERATIVE tense description of the change:");
    let longer_description = read_multiline(
        "Provide a LONGER description of the change (optional). Use \"|\" to break new line:\n",
    );
    let breaking_changes =
        read_multiline("List any BREAKING CHANGES (optional). Use \"|\" to break new line:\n");

    let (issue_prefix, issue_refs) = read_issues();

    let mut commit_message = format!("{}({}): {}", chosen_type.key, final_scope, desc);

    if !longer_description.is_empty() {
        commit_message.push_str(&format!("\n\n{}", longer_description));
    }
    if !breaking_changes.is_empty() {
        commit_message.push_str(&format!("\n\nBREAKING CHANGE: {}", breaking_changes));
    }
    if !issue_prefix.is_empty() {
        commit_message.push_str(&format!("\n\n{} {}", issue_prefix, issue_refs));
    }

    // Hasil akhir
    execute!(
        stdout,
        Print("\n"),
        ResetColor,
        Print("###--------------------------------------------------------###\n"),
        SetForegroundColor(Color::DarkGreen),
        Print("✔ Generated Git Commit Message:\n"),
        ResetColor,
        SetForegroundColor(Color::White),
        Print(format!("{}\n", commit_message)),
        ResetColor,
        Print("###--------------------------------------------------------###\n"),
    )?;

    terminal::disable_raw_mode()?;

    let mut confirm = String::new();
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGreen),
        Print("? "),
        ResetColor,
        SetForegroundColor(Color::White),
        Print("Are you sure you want to proceed with the commit above? (Y/n) "),
        ResetColor
    )?;
    io::stdout().flush()?;
    io::stdin().read_line(&mut confirm)?;
    let confirm = confirm.trim().to_lowercase();

    if confirm == "n" || confirm == "no" {
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print("❌ Commit canceled by user.\n"),
            ResetColor
        )?;
        return Ok(());
    }

    if confirm != "y" && confirm != "yes" {
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print("❌ Commit canceled by user.\n"),
            ResetColor
        )?;
        return Ok(());
    }

    // Eksekusi git commit
    let status = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .status()
        .expect("failed to run git commit");

    if !status.success() {
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print("❌ git commit failed\n"),
            ResetColor
        )?;
    } else {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGreen),
            Print("✅ Commit successful!\n"),
            ResetColor
        )?;
    }

    Ok(())
}
