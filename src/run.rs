use tokio::fs;
use tokio::process;
use tokio::sync::{mpsc, mpsc::Sender};
use tokio::time::{sleep, Duration};

use std::env;

use crossterm::terminal::enable_raw_mode;

pub type DynResult = Result<(), Box<dyn std::error::Error>>;

pub async fn run() -> DynResult {
    enable_raw_mode()?;

    let (char_tx, mut char_rx) = mpsc::channel::<char>(1);
    let input_thread = tokio::spawn(async { input_reader(char_tx).expect("read input") });

    'main: loop {
        // wait for update commands
        tokio::select!(
            Some(res) = char_rx.recv() => {
                println!("Keypress: {}", res);
                match res {
                    'u' => perform_update().await?,
                    'c' => restart().await?,
                    'r' => {
                        restart().await?;
                        break 'main;
                    },
                    'q' => break 'main,
                    _ => ()
                }
            }
            _ = sleep(Duration::from_millis(5000)) => {
                println!("Press (q)uit, (u)pdate, (r)estart, (c)reate another instance here")
            }
        )
    }
    char_rx.close();
    input_thread.await?;

    Ok(())
}

async fn perform_update() -> DynResult {
    let current_exe = env::current_exe()?;
    let mut update_original = current_exe.clone();
    update_original.set_file_name("self-restart-updated.exe");
    let mut update_clone = update_original.clone();
    update_clone.set_file_name("self-restart-temp.bin");
    let mut update_temp = update_original.clone();
    update_temp.set_file_name("self-restart-temp.bin.tmp");

    // Create a clone to not lose the original
    fs::copy(&update_original, &update_clone).await?;

    println!(
        "Prepare for update: {:?} to be replaced with {:?}",
        current_exe, update_clone
    );

    self_update::Move::from_source(update_clone.as_path())
        .replace_using_temp(update_temp.as_path())
        .to_dest(current_exe.as_path())?;
    Ok(())
}

async fn restart() -> DynResult {
    process::Command::new(env::current_exe()?).spawn()?;
    Ok(())
}

fn input_reader(char_tx: Sender<char>) -> DynResult {
    use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
    loop {
        if poll(Duration::from_millis(100))? {
            let input_char = read().expect("read input event");
            match input_char {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(code),
                    ..
                }) => match char_tx.try_send(code) {
                    Err(mpsc::error::TrySendError::Closed(_)) => break,
                    _ => (),
                },
                _ => (),
            }
        } else {
            // timeout expired, check if we should stop
            if char_tx.is_closed() {
                break;
            }
        }
    }

    Ok(())
}
