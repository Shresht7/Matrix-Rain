use std::{io::Write, time::Duration};

use clap::Parser;
use crossterm::{
    cursor,
    style::{style, Stylize},
    terminal, QueueableCommand,
};

mod ansi;
mod config;
mod entity;
mod events;
mod matrix;
mod stream;
mod symbols;
mod utils;

//  ====
//  MAIN
//  ====

/// The main entrypoint of the application
fn main() {
    //  Parse command-line arguments as the configuration
    let config = config::Config::parse();
    // Run the main logic with the given command-line arguments
    match run(&config) {
        Err(e) => {
            eprintln!("{}", style(format!("Error: {e}")).red());
            std::process::exit(1)
        }
        Ok(_) => std::process::exit(0),
    }
}

/// Run the main logic of the application
fn run(config: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
    // Get a reference to stdout
    let mut stdout = std::io::stdout();

    //  Get Terminal Window Size to determine the number of rows and columns
    let (columns, rows) = terminal::size()?;

    //  Instantiate streams
    let mut matrix = matrix::Matrix::new(rows, columns, &config);

    // Setup the terminal before running the application
    setup(&mut stdout)?;

    //  Render the Matrix-Rain on screen
    loop {
        //  Render each stream
        matrix.render(&config);

        // Handle events
        if let events::Action::Exit = events::handle_events()? {
            break;
        }

        //  Sleep for 1/FPS seconds
        std::thread::sleep(Duration::from_millis(1000 / config.fps));
    }

    // Cleanup the terminal after the application stops
    cleanup(&mut stdout)?;

    Ok(())
}

/// Prepares the terminal by switching to the alternate screen and clearing it.
/// Also moves the cursor to the top before hiding it from view.
/// Registers a panic-hook to automatically call the `cleanup` function
fn setup(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .queue(cursor::Hide)?
        .flush()?;

    // Create a custom hook to handle graceful cleanup of the terminal when panicking
    let original_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let mut stdout = std::io::stdout();
        // Intentionally ignore errors here since we're already in a panic!
        let _ = cleanup(&mut stdout);
        original_panic(info);
    }));

    Ok(())
}

/// Restores terminal to its original state by leaving alternate screen,
/// showing the cursor, and disabling raw mode.
fn cleanup(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    stdout
        .queue(terminal::LeaveAlternateScreen)?
        .queue(cursor::Show)?
        .flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}
