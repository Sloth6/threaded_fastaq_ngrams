#![feature(std_misc)]
#![allow(deprecated,unused_variables,unused_must_use)]

use std::thread::Thread;
use std::sync::mpsc::{SyncSender, Sender, Receiver, channel};
use std::sync::{Arc,Mutex};

fn main() {
	let num_threads = 3;
  let (result_tx, result_rx): (Sender<usize>, Receiver<usize>) = channel();

  let channels: Vec<_> = ((0..num_threads).map(|i| {
    let (tx, rx): (Sender<Option<usize>>, Receiver<Option<usize>>) = channel();
    Arc::new((Mutex::new(tx), Mutex::new(rx)))
  }).collect());

  for i in 0..num_threads {
    let threadtx = result_tx.clone();
    let threadChannel = channels[i].clone();
    println!("thread {:?}", i);
    Thread::spawn(move || {
      let (_, ref mutex) = *threadChannel;
      let ref rx = mutex.lock().unwrap();
      loop {
        match rx.recv().unwrap() {
          Some(j) => println!("{:?}", j),
          None => {
            threadtx.send(1).unwrap();
            break
          },
        }
      }
    });
  }

  for line in 0..num_threads {
    let foo = channels[line].clone();
    let (ref mutex, _) = *foo;
    let ref tx = mutex.lock().unwrap();
    tx.send(Some(1)).unwrap();
    tx.send(None).unwrap();
  }

  for _ in 0..num_threads {
    //block the current thread if there no messages available
    println!("{:?}", result_rx.recv());
  }
  println!("done!");
}