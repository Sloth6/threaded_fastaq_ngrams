#![feature(std_misc)]
#![allow(deprecated,unused_variables,unused_must_use)]

use std::thread::Thread;
use std::sync::mpsc::{SyncSender, Sender, Receiver, channel};
use std::sync::{Arc,Mutex};

use std::old_io::File;
use std::os;
use std::old_io::BufferedReader;
use std::collections::HashMap;

fn main() {
  let num_threads = 3;


  let (result_tx, result_rx): (Sender<usize>, Receiver<usize>) = channel();
  let channels: Vec<_> = ((0..num_threads).map(|i| {
    let (tx, rx): (Sender<Option<_>>, Receiver<Option<_>>) = channel();
    Arc::new((Mutex::new(tx), Mutex::new(rx)))
  }).collect());

  for i in 0..num_threads {
    let threadtx = result_tx.clone();
    let threadChannel = channels[i].clone();
    println!("Starting thread {:?}", i);
    Thread::spawn(move || {
      let (_, ref mutex) = *threadChannel;
      let ref rx = mutex.lock().unwrap();
      let mut myCounter = 0;
      loop {
        match rx.recv().unwrap() {
          Some(j) => myCounter += 1,//println!("{:?}", j)
          None => {
            threadtx.send(myCounter).unwrap();
            println!("Ending thread {:?}", i);
            break
          },
        }
      }
    });
  }

  // let path = Path::new("/Users/joelsimon/Documents/Immufind/SequenceAnalysis/data/test.txt".to_string());
  let path = Path::new("/Users/joelsimon/Documents/Immufind/SequenceAnalysis/data/HD3_IgG1sorted_S1_L001_I1_001.fastq".to_string());
  let file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", path.display(), why.desc),
    Ok(file) => file,
  };
  let mut reader = BufferedReader::new(file);
  let mut lines = reader.lines().filter_map(|result| result.ok());
  
  let mut count = 0;
  loop {
    match ( lines.next(), lines.next(), lines.next(), lines.next() ) {
      (Some(name), Some(seq), _, _) => {
        let i = count % num_threads;
        let foo = channels[i].clone();
        let (ref mutex, _) = *foo;
        let ref tx = mutex.lock().unwrap();
        let mut nuc_seq = vec![];
        for c in seq.chars() {
          nuc_seq.push(c);
        }
        tx.send(Some(nuc_seq)).unwrap();
      },
      _ => { break }
    }
    count+=1;
  }

  for line in 0..num_threads {
    let foo = channels[line].clone();
    let (ref mutex, _) = *foo;
    let ref tx = mutex.lock().unwrap();
    // tx.send(Some("hello")).unwrap();
    tx.send(None).unwrap();
  }

  for _ in 0..num_threads {
    //block the current thread if there no messages available
    println!("{:?}", result_rx.recv());
  }
  println!("done!");
}