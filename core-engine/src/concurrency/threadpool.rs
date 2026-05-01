use crossbeam::channel::{Receiver, Sender};

use crate::concurrency::{RenderJob, RenderJobResult};
use crate::sampler::Sampler;

use super::worker::RenderingWorker;

pub struct Threadpool {
    workers: Vec<RenderingWorker>,
    job_tx: Option<Sender<RenderJob>>,
}

impl Threadpool {
    pub fn new(size: usize) -> (Self, Receiver<RenderJobResult>) {
        assert!(size > 0);
        let (
            job_tx,
            job_rx)
        = crossbeam::channel::unbounded::<RenderJob>();

        let (result_tx, result_rx) = crossbeam::channel::unbounded();

        let mut workers = vec![];

        for id in 0..size {
            workers.push(RenderingWorker::new(id, job_rx.clone(), result_tx.clone()));
        }

        let tp = Self {
            workers,
            job_tx: Some(job_tx),
        };

        (tp, result_rx)
    }

    pub fn execute<T>(&self, render_func: T)
    where
        T: FnOnce(&mut Sampler) -> RenderJobResult + Send + 'static,
    {
        let job = Box::new(render_func);
        self.job_tx.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        if let Some(job_tx) = self.job_tx.take() {
            drop(job_tx);
        }
        for worker in &mut self.workers {
            worker.join_and_stop();
        }
    }
}
