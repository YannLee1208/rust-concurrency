use std::{thread, time::Duration};

use anyhow::Result;
use rand::Rng;
use rust_concurrecy::Metrics;

const PROCEDUER: usize = 4;
const REQUESTER: usize = 3;

fn main() -> Result<()> {
    let metrics = Metrics::new();
    for idx in 0..PROCEDUER {
        proceduer(idx, metrics.clone())?;
    }

    for _ in 0..REQUESTER {
        requester(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        let snapshot = metrics.snapshot()?;
        println!("{:?}", snapshot);
    }
}

fn proceduer(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..1000)));
            let page = format!("req.page.{}", idx);
            metrics.desc(&page)?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });

    Ok(())
}

fn requester(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..2000)));
            let page = format!("req.page.{}", rng.gen_range(0..PROCEDUER));
            metrics.inc(&page)?;
        }

        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });

    Ok(())
}
