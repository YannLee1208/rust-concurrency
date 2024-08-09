use anyhow::{anyhow, Result};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

const NUM_PRODUCES: usize = 4;

#[derive(Debug)]
#[allow(dead_code)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCES {
        let tx = tx.clone();
        thread::spawn(move || produce(i, tx));
    }

    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer : {:?}", msg);
        }
        println!("Consumer left");
    });

    consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    Ok(())
}

fn produce(idx: usize, tx: Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        thread::sleep(Duration::from_millis(1000));
        if rand::random::<u8>() % 10 == 0 {
            println!("Producer {:?} left", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
