use std::thread;
use std::sync::mpsc;

// ANCHOR: pool
pub struct ThreadPool {
    threads: Vec<Worker>
}

impl ThreadPool {
    pub fn new(n: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(n);

        for i in 0..n {
            workers.push(Worker::new(i));
        }

        ThreadPool { threads: workers }
    }

    pub fn execute<F>(&mut self, f: F) -> Worker
    where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        let w = self.threads.pop().expect("No more threads!");
        println!("execute sending work to {}", w.id);
        w.sender.send(Task::Work(job)).unwrap();
        w
    }

    pub fn wait(&mut self, w: Worker) {
        if let Ok(rc) = w.receiver.recv() {
            if !rc {
                println!("worker failed?!");
            }
        }
        println!("worker {} is finished", w.id);
        self.threads.push(w);
    }

    pub fn drain(&mut self) {
        while let Some(w) = self.threads.pop() {
            w.sender.send(Task::Quit).unwrap();
            w.thread.join().unwrap();
        }
    }    
}
// ANCHOR_END: pool

// ANCHOR: worker
type Job = Box<dyn FnOnce() + Send>;

enum Task {
    Quit,
    Work(Job),
}

pub struct Worker {
    id: usize,
    sender: mpsc::Sender<Task>,
    receiver: mpsc::Receiver<bool>,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let (thread_sender, thread_receiver) = mpsc::channel(); 
        let (mgr_sender, mgr_receiver) = mpsc::channel(); 
        let thread = thread::spawn(move || {
            loop {
                let work: Task = thread_receiver.recv().unwrap();
                match work {
                    Task::Quit => {
                        break;
                    },
                    Task::Work(job) => {
                        job();
                    },
                }
                mgr_sender.send(true).expect("failed manager send");
            }
        });

        Worker { id, sender: thread_sender, receiver: mgr_receiver, thread }
    }
}
// ANCHOR_END: worker
