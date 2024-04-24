use std::path::Path;
use std::sync::mpsc::channel;

use notify::Event;
use notify::Result;
use notify::Watcher;

fn main() -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| match res {
        Ok(event) => {
            println!("new event {:?}", event);
            if let Err(e) = tx.send(event) {
                println!("error sending event for {:?}", e);
            }
        }
        Err(e) => println!("err       {:?}", e),
    })?;

    watcher.watch(Path::new("."), notify::RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => println!("Received: {:?}", event),
            Err(e) => {
                println!("Receive error: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
