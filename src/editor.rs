use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders};
use tui::Terminal;
use tui_textarea::{Input, Key, TextArea};

pub fn editor(for_edit: Option<String>) -> io::Result<String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Write or edit your doc"),
    );

    if let Some(ref stri) = for_edit {
        let ss: Vec<_> = stri.split('\n').collect();
        for line in ss.iter() {
            textarea.insert_str(*line);
            textarea.insert_newline();
        }
    }

    loop {
        term.draw(|f| {
            f.render_widget(textarea.widget(), f.size());
        })?;
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            input => {
                textarea.input(input);
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;
    let mut final_string = String::new();
    for line in textarea.lines() {
        let ss = line.to_owned() + "\n";
        final_string.push_str(&ss);
    }
    Ok(final_string)
}
