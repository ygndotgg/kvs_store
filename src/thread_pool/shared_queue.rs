use std::{
    panic::{self, AssertUnwindSafe},
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

enum Message {
    Terminate,
    Job(Box<dyn FnOnce() + Send + 'static>),
}

struct Worker {
    handle: Option<JoinHandle<()>>,
}

pub struct SharedQueueThreadPool {
    sender: mpsc::Sender<Message>,
    workers: Vec<Worker>,
}

impl super::ThreadPool for SharedQueueThreadPool {
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::Job(Box::new(job))).unwrap();
    }
    fn new(threads: u32) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let (tx, rx) = mpsc::channel::<Message>();
        let mut workers = Vec::with_capacity(threads as usize);
        let receiver = Arc::new(Mutex::new(rx));
        for _ in 0..threads {
            let receiver = receiver.clone();
            let d = thread::spawn(move || loop {
                let item = receiver.lock().unwrap().recv();
                match item {
                    Ok(Message::Job(j)) => {
                        let _ = panic::catch_unwind(AssertUnwindSafe(j));
                    }
                    Ok(Message::Terminate) | Err(_) => break,
                }
            });
            workers.push(Worker { handle: Some(d) });
        }
        Ok(SharedQueueThreadPool {
            sender: tx,
            workers,
        })
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            let _ = self.sender.send(Message::Terminate);
        }
        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                let _ = handle.join();
            }
        }
    }
}
