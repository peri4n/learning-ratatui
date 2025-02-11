use std::io::{stdout, Stdout};

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

pub use app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

pub mod app;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = init()?;
    let result = App::default().run(&mut terminal);
    if let Err(err) = ratatui::try_restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    result
}

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> std::io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    set_panic_hook();
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore(); // ignore any errors as we are already failing
        hook(panic_info);
    }));
}

/// Restore the terminal to its original state
pub fn restore() -> std::io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
