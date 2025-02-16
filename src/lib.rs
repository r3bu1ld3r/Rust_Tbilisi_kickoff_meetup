#[cfg(loom)]
use loom::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[cfg(not(loom))]
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    pub fn new(
        name: String,
        left_fork: Arc<Mutex<Fork>>,
        right_fork: Arc<Mutex<Fork>>,
        thoughts: mpsc::Sender<String>,
    ) -> Self {
        Self {
            name,
            left_fork,
            right_fork,
            thoughts,
        }
    }

    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        match (self.left_fork.lock(), self.right_fork.lock()) {
            (Ok(_l), Ok(_r)) => {
                println!("{} is eating...", &self.name);
            }
            (_, _) => {
                println!("{} couldn't get both forks", &self.name);
            }
        }
    }
}

#[cfg(loom)]
static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato"];
#[cfg(not(loom))]
static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

pub fn solution(correct: bool) {
    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    let (tx, rx) = mpsc::channel();

    let philosophers = PHILOSOPHERS
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let mut left_fork = Arc::clone(&forks[i]);
            let mut right_fork = Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]);
            if correct {
                if i == forks.len() - 1 {
                    println!("swapping forks");
                    std::mem::swap(&mut left_fork, &mut right_fork);
                }
            }
            Philosopher::new(name.to_string(), left_fork, right_fork, tx.clone())
        })
        .collect::<Vec<_>>();

    for philosopher in philosophers {
        thread::spawn(move || {
            if cfg!(loom) {
                for _ in 0..10 {
                    philosopher.eat();
                    philosopher.think();
                }
            } else {
                for _ in 0..100 {
                    philosopher.eat();
                    philosopher.think();
                }
            }
        });
    }

    drop(tx);
    #[cfg(loom)]
    for _ in 0..30 {
        match rx.recv() {
            Ok(thought) => println!("{thought}"),
            Err(e) => {
                println!("Error: {e}");
                break;
            }
        }
    }

    #[cfg(not(loom))]
    for thought in rx {
        println!("{thought}");
    }

    println!("All done!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn trigger_deadlock() {
        use std::sync::mpsc::channel;
        use std::time::Duration;

        let (done_tx, done_rx) = channel();
        std::thread::spawn(move || {
            solution(false);
            done_tx.send(()).unwrap();
        });
        match done_rx.recv_timeout(Duration::from_secs(5)) {
            Ok(_) => {}
            Err(e) => panic!("Test timed out, potential deadlock detected: {}", e),
        }
    }

    #[test]
    fn run_correct_solution() {
        solution(true);
    }

    #[test]
    #[should_panic]
    fn trigger_deadlock_with_loom() {
        loom::model(|| solution(false));
    }

    #[test]
    fn run_correct_solution_with_loom() {
        loom::model(|| solution(true));
    }
}
