use commitz::commit::{
    reader::{read_commit_type, read_desc, read_issues, read_multiline, read_scope},
    renderer::{render_commit, render_scope},
    types::RenderCommit,
    validator::{confirm_question, ensure_git_repo, ensure_staged_files},
};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, EnterAlternateScreen},
};
use std::{
    env,
    io::{self, stdout},
    process::Command,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1] == "-v" || args[1] == "--version") {
        println!("commitz {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    ensure_git_repo()?;
    ensure_staged_files()?;
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let chosen_type = read_commit_type()?;
    let chosen_scope = render_scope()?;
    let final_scope = read_scope(&chosen_scope)?;

    let desc = read_desc()?;
    let longer_description = read_multiline(
        "Provide a LONGER description of the change (optional). Use \"|\" to break new line:\n",
    )?;

    let mut breaking_changes = String::new();

    if chosen_scope == "custom" {
        breaking_changes =
            read_multiline("List any BREAKING CHANGES (optional). Use \"|\" to break new line:\n")?;
    }
    let (issue_prefix, issue_refs) = read_issues()?;
    let mut commit_message = format!("{}{}: {}", chosen_type.key, final_scope, desc);

    if !longer_description.is_empty() {
        commit_message.push_str(&format!("\n\n{}", longer_description));
    }
    if !breaking_changes.is_empty() {
        commit_message.push_str(&format!("\n\nBREAKING CHANGE: {}", breaking_changes));
    }
    if !issue_prefix.is_empty() {
        commit_message.push_str(&format!("\n\n{} {}", issue_prefix, issue_refs));
    }

    render_commit(
        &mut stdout,
        &RenderCommit::new(
            chosen_type.key,
            final_scope,
            desc,
            longer_description,
            breaking_changes,
            issue_prefix,
            issue_refs,
        ),
    )?;

    if confirm_question("Are you sure you want to proceed with the commit above?")? {
        execute!(stdout, ResetColor, Print("\nCommit confirmed!\n"))?;
    } else {
        terminal::disable_raw_mode()?;
        return Ok(());
    }

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
            Print("\n✅ Commit successful!\n"),
            ResetColor,
        )?;
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
