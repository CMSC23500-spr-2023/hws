use std::f64::consts::PI;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

pub trait Geometry {
    fn get_area(&self) -> f64;
    fn get_name(&self) -> String;
}

pub struct Rectangle {
    pub length: f64,
    pub width: f64,
}

impl Geometry for Rectangle {
    panic!("TODO milestone primer-mod5");
}


pub struct Circle {
    pub radius: f64,
}

impl Geometry for Circle {
    panic!("TODO milestone primer-mod5");
}

struct Counter {
    count: i32
}

fn incr(counter: &Arc<Mutex<Counter>>) {
    panic!("TODO milestone primer-mod5");
}

fn counter() {
    // declare a counter wrapped in a mutex
    // spawn a thread to call incr() 50 times
    // in main thread call incr() 50 times 
    panic!("TODO milestone primer-mod5");
}

/*
 *  Modify the following function to use RwLocks. Run both blocks, one with standard mutexes
 *  and the other with RwLocks. What differences do you observe in their behavior/output? Does
 *  this match your understanding of how Read/Write locks work?
 *
 *  No submission is needed for this exercise.
 */
fn read_write() {
    let lock = Arc::new(Mutex::new(0));
    let mut handles = Vec::with_capacity(10);

    for _i in 0..10 {
        let reader_lock = lock.clone();
        let reader = thread::spawn(move || {
            for _j in 0..20 {
                let r = reader_lock.lock().unwrap();
                println!("Read value as {}", *r);
            }
        });
        handles.push(reader)
    }

    for _j in 0..20 {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("Incremented value by 1 to {}", *val);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

