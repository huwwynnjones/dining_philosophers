extern crate rand;

use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    #[derive(Clone, Copy)]
    struct Fork {};

    struct Philosopher {
        name: String,
        left: Option<Fork>,
        right: Option<Fork>,
        times_eaten: i32,
    };

    impl Philosopher {
        fn new(name: &str) -> Philosopher {
            Philosopher {
                name: name.to_string(),
                left: None,
                right: None,
                times_eaten: 0,
            }
        }

        fn try_take_fork(&mut self, table: &mut Table) -> () {
            match table.try_take_fork() {
                Some(fork) => {
                    if self.left.is_none() {
                        self.left = Some(fork)
                    } else if self.right.is_none() {
                        self.right = Some(fork)
                    }
                }
                None => {
                    println!("{} says no forks on the table", self.name);
                }
            }
        }

        fn can_eat(&self) -> bool {
            self.left.is_some() && self.right.is_some()
        }

        fn eat(&mut self) -> () {
            println!("{} is eating for the {} time", self.name, self.times_eaten);
            thread::sleep(Duration::from_secs(thread_rng().gen_range(0, 2)));
            self.times_eaten += 1
        }

        fn is_hungry(&self) -> bool {
            self.times_eaten < 5
        }

        fn think(&self) -> () {
            println!("{} is thinking", self.name);
            thread::sleep(Duration::from_secs(thread_rng().gen_range(0, 2)))
        }

        fn return_fork(&mut self, table: &mut Table) -> () {
            if self.left.is_some() {
                table.return_fork(self.left.take().unwrap())
            } else if self.right.is_some() {
                table.return_fork(self.right.take().unwrap())
            }
        }

        fn return_both_forks(&mut self, table: &mut Table) -> () {
            self.return_fork(table);
            self.return_fork(table)
        }
    }

    #[derive(Clone)]
    struct Table {
        forks: Vec<Fork>,
    }

    impl Table {
        fn new(nmb_of_forks: i32) -> Table {
            let mut forks = Vec::new();
            for _ in 0..nmb_of_forks {
                forks.push(Fork {})
            }
            Table { forks }
        }

        fn try_take_fork(&mut self) -> Option<Fork> {
            self.forks.pop()
        }

        fn return_fork(&mut self, fork: Fork) -> () {
            self.forks.push(fork)
        }
    }

    let protected_table = Arc::new(Mutex::new(Table::new(4)));

    println!("Starting dinner");

    let mut handles = Vec::new();

    for name in ["John", "Ben", "Dave", "Ron"].iter() {
        let lock = Arc::clone(&protected_table);
        let mut philosopher = Philosopher::new(name);
        let handle = thread::spawn(move || {
            while philosopher.is_hungry() {
                if philosopher.can_eat() {
                    philosopher.eat();
                    let mut tbl = lock.lock().unwrap();
                    philosopher.return_both_forks(&mut tbl)
                } else {
                    {
                        let mut tbl = lock.lock().unwrap();
                        philosopher.try_take_fork(&mut tbl);
                        philosopher.try_take_fork(&mut tbl)
                    }
                    philosopher.think()
                }
            }
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
