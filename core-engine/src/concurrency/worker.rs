use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{Receiver, Sender};

use crate::concurrency::{RenderJob, RenderJobResult};
use crate::sampler::Sampler;

pub struct RenderingWorker {
    thread: Option<JoinHandle<()>>,
}

impl RenderingWorker {
    pub fn new(id: usize, job_rx: Receiver<RenderJob>, result_tx: Sender<RenderJobResult>) -> Self {
        let thread = thread::spawn(move || {
            let mut sampler = Sampler::new();
            loop {
                match job_rx.recv() {
                    Ok(job) => {
                        let acc = job(&mut sampler);
                        if let Err(e) = result_tx.send(acc) {
                            #[cfg(debug_assertions)]
                            dbg!(
                                "Worker {} failed to send result: receiver disconnected. Err : {:?}",
                                id, e
                            );
                            break;
                        }
                    }
                    Err(_) => {
                        #[cfg(debug_assertions)]
                        dbg!("Worker {id} disconnected; Shutting down..");
                        break;
                    }
                }
            }
        });

        Self {
            thread: Some(thread),
        }
    }

    pub fn join_and_stop(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = thread.join().unwrap();
        }
    }
}
