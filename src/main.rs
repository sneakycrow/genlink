extern crate notify;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;

fn main() {
  let (tx, rx) = channel();

  let mut watcher = raw_watcher(tx).unwrap();

  watcher.watch("/Users/zachsohovich/Desktop", RecursiveMode::Recursive).unwrap();

  loop {
    match rx.recv() {
      Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
        println!("{:?} {:?} ({:?})", op, path, cookie)
      },
      Ok(event) => println!("broken event: {:?}", event),
      Err(e) => println!("watch error: {:?}", e),
    }
  }
}