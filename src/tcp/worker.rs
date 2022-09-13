use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use super::fn_box::FnBox;

type Job = Box<dyn FnBox + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}

