# [Module 5](#module-5)
- [Traits](#traits)
- [Shared Ownership (rc / arc / mutex / rwlock)](#shared-ownership)


## Traits

A [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) defines behavior (i.e., functions). This is similar to interfaces from other languages. You can associate behavior with types via Traits. When you do that, you are stating that those types implement the behavior defined by the Trait. Let's make all this concrete. Consider a trait called ```Summary```.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

We can then associate this Trait with different types. When we do that, we also implement the functions, unlike above, where we only declare them. For example,

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

As you can see, we implemented the same trait, ```Summary```, on 2 different structs: NewsArticle, and Tweet. We are saying that although NewsArticle and
Tweet are two different types, they both implement the Summary behavior. In addition, NewsArticle and Tweet can implement other Traits and have a default implementation still (e.g. `impl Tweet`)

## Threading in Rust

Similar to C, C++ and Java, Rust provides the ability to write multi-threaded programs which allows for concurrent execution of program code, and threads have the ability to read and write shared data, as we shall see soon. 

Threading in rust is handled via the `std::thread` library, which provides the 
ability to `spawn` threads. The thread `thread::spawn` function call takes in a function `closure`, which is an anonmyous function with associated context that can be saved in a variable and passed to other functions. The following example spawns a single thread which prints 10 lines, while the main thread prints 5 lines, and these can run concurrently. The closure is defined as a parameter to `thread::spwan` using the `|| {}` notation. 

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

```

[Read more about closures in the Rust book](https://doc.rust-lang.org/book/ch13-01-closures.html)

The example also shows the use of `thread:sleep` to force a thread to stop its execution for sometime. 

A call to the `thread::spawn` function returns a variable of type `JoinHandle`. This variable gives us a handle to the thread which we can use to interact with the thread and call the thread `join()` function. `join` forces the calling thread to wait for the execution of the thread pointed to by the handle to finish executing. In rust, when the main thread is done executing, the program exits, irrespective of any threads that have not finished executing, unless you force the execution to wait until all spawned threads have been joined using `join`. 

[Read more about Threads in the Rust book](https://doc.rust-lang.org/book/ch16-01-threads.html)

## Shared Ownership

One of the unique characteristics of Rust as a programming language, [is its notion of ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html). In Rust, if you want to share a variable, (e.g., passing it as argument, using it to compute another variable, or using it as the return value of a function), you will have to think about ownership. There are a few concepts that we use in crustyDB and that are related to how we handle references in Rust. We give an overview of those here.

A key thing to keep in mind for the discussion that follows is that whenever you allocate a variable, you want to keep track of all existing references to that variable so that when there are no more references left, you can deallocate the variable. While in some languages, such as Java and C#, there is a garbage collector that takes care of counting and handling references automatically and transparently for you, in Rust there is no garbage collector. This means that someone must take care of incrementing the reference counter when new references are created, and ensuring the data is dropped only when the reference counter is zero. In Rust, the compiler will take care of doing this for you, as long as you give it enough information. If you create a reference in Rust, it'll get deallocated, by default, when the reference goes out of scope.  If you want to keep that reference around, (e.g., because you've shared it), how do you achieve that? The types we discuss below give you some primitives and tools to handle this. While we do not expect you to understand all the details of these types right now, we hope this guide will help you become a bit more familiar with them, so you will understand and interpret them when you find them in crustyDB.


## Rc 

'[Rc](https://doc.rust-lang.org/book/ch15-04-rc.html)' stands for reference counted. 'Rc' is a type, ```Rc<T>```, that provides *shared* ownership of a value of type T. ```Rc<T>``` automatically deferences to T, and you can call any of T's methods on a value of type ```Rc<T>```.  For example, consider a scenario where we have ```Objects``` that are owned by a given ```Owner```. We want to have our ```Objects``` point to their ```Owner```. We can't do this with ownership because one or more ```Objects``` could belong to the same owner. In this scenario, ```Rc``` allows us to share ownership over multiple ```Objects```. 

```rust
use std::rc::Rc;

struct Owner {
    name: String,
    // ...other fields
}

struct Object {
    id: i32,
    owner: Rc<Owner>,
    // ...other fields
}

fn main() {
    // Create a reference-counted `Owner`.
    let object_owner: Rc<Owner> = Rc::new(
        Owner {
            name: "name".to_string(),
        }
    );

    // Create `Object`s belonging to `object_owner`. Cloning the `Rc<Owner>`
    // gives us a new pointer to the same `Owner` allocation, incrementing
    // the reference count in the process.
    let object1 = Object {
        id: 1,
        owner: Rc::clone(&object_owner),
    };
    let object2 = Object {
        id: 2,
        owner: Rc::clone(&object_owner),
    };


    println!("Object {} owned by {}", object1.id, object1.owner.name);
    println!("Object {} owned by {}", object2.id, object2.owner.name);

}
```

## Arc 

```Arc``` stands for Atomic Reference Counted, and is similar to ```Rc```, ```Arc``` lets you share data across different owners. In contrast to Rc, Arc allows you to share references across *threads* and ensures that the reference lives as long as the last owner survives--as opposed to the reference being deallocated when it gets out of scope. A quick way of choosing between Arc and Rc is the following: will you use the reference across threads? if the answer is yes, you probably want to use Arc, if the answer is no then you probably want to use Rc.

# Concurrent data access in Rust

If you build a multi-threaded application, and multiple threads want to access the same data, many things can go wrong. You will want to synchronize and control the access to that data. Rust provides several types to indicate how you want to handle concurrency. We explain a few of those types below.

First, let us take a look at how you can provide a thread with ownership of a variable:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

The example contains a spawned thread which accesses a vector `v` inside the thread. Note that we have used the keyword `move` along with the function closure to indicate to the rust compiler to automatically take ownership any used values from the outer context. Once a value has been owned by the thread, it can no longer be used in the calling context (i.e. the main thread). However, we can use specialized primitives to share and protect access to data across threads, as will be demonstrated next. 

[Read more about move closures in the Rust book](https://doc.rust-lang.org/book/ch16-01-threads.html#using-move-closures-with-threads)

## mutex 

```Mutex``` is a mutual exclusion primitive useful for protecting shared data. With a mutex, you can make sure that only 1 thread access a piece of data at a time. Anything that wants to operate on the variable held by a mutex, must first acquire the mutex (lock). The mutex is valid for the scope it is defined/acquired. The mutex can be initialized or created with a ```new``` constructor.  For example: 

```rust
use std::sync::{Arc, Mutex}; 
use std::thread; 
use std::time::Duration; 
fn main()  
{ 
    let primes = Arc::new(Mutex::new(vec![1,2,3,5,7,9,13,17,19,23])); 
 
    for i in 0..10  
    { 
        let primes = primes.clone(); 
        thread::spawn(move ||  
        {  
            let mut data = primes.lock().unwrap(); 
            data[0] += i;  
        }); 
    } 
    thread::sleep(Duration::from_millis(50)); 
} 
```

We are creating 10 threads in the example above. Each thread has a reference to the vector primes. Because we have multiple references, we tell Rust to reference count them. Because these references are shared across different threads, we use Arc to indicate that. 

In addition, we want only 1 thread accessing the data at a time. To achieve that, we create a Mutex on the primes variable.

## RwLock

Imagine you have many threads that are reading a variable. You may let them read concurrently (as long as they do not modify the data, this may work for your application). However, if one wants to modify the contents (i.e., write), you want to make sure nobody is reading at the same time. If you were to use a Mutex only one thread could access the data, whether for reading or writing. To allow for concurrent reads, Rust provides a RwLock.

```RwLock``` stands for Reader-Writer lock. This type of lock allows a number of readers or at most one writer at any point in time. The write portion of this lock allows modification of the underlying data and the read portion allows for read-only shared access. For example:

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);

// many reader locks can be held at once
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();

} // read locks are dropped at this point

// only one write lock may be held, however
{
    let mut w = lock.write().unwrap();
    *w += 1;
} // write lock is dropped here
```

In this example, RwLock allowed us to have many reader locks, while only allowing access to one write lock. Most of the time when you have a RwLock, it will be wrapped in an Arc, as it meant to be shared across threads.

## Other Concurrency Primitives/ Tools

We are not covering them here, but in addition to RwLocks, Rust uses AtomicPrimitives for safe concurrent access along with channels to use message passing between threads.

## Interior Mutability

From the Rust book '*Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules.*'  Using a variable like `Arc<RwLock<T>>` can allow you to use interior mutability on a struct, and within CrustyDB there will be points you may want to use such an approach.

## Testing

We  will cover this with the start of CrustyDB, but if you are not familiar with unit tests or integration tests, you will want to read the documentation on [testing](https://doc.rust-lang.org/book/ch11-00-testing.html).

## Module 5 Exercises

Complete the following exercises and put all code in ```solution.rs```. Function and struct declarations are already provided to you, so put the code needed to complete the exercises there. To test, can either write your own tests or use ```main.rs```, but neither your tests or anything you write in ```main.rs``` will be used for grading: only what's in ```solution.rs```.

1. Define a trait called *Geometry* that has two methods: *get_area* and *get_name*. Then implement that trait on two types - *Rectangle* and *Circle*. *get_area* should return a float (use f64). *get_name* should return the name of the shape as a String. You can import PI via *use std::f64::consts::PI*. 

2. In this exercise, you will need to implement a two-thread counter that counts from 0 to 100. Below is the skeleton of the counter program. Read the comments and complete as needed.

```rust

use std::thread;

struct Counter {
    count: i32
}

// define a function incr() to increment count in Counter by 1
fn incr() {}

fn counter() {
    // declare a counter wrapped in a mutex

    // spawn a thread here to call incr() 50 times
    let handle = thread::spawn(move|| {
        for _i in [??] {
            println!("thread spawned count {}", [??]);
        }
    });
   
    // in the main thread, call incr() 50 times
    for _i in [??] {
        println!("thread main count {}", [??]);
    }
    handle.join().unwrap();
}
```

3. Modify the following code block to use RwLocks. Run both blocks, one with standard mutexes and the other with RwLocks. What differences do you observe in their behavior/output? Does this match your understanding of how Read/Write locks work? You don't need to submit anything for this exercise, just get experience with RwLocks and observe the difference between RwLocks and Mutexes.

```rust
use std::sync::{Arc, RwLock, Mutex};
use std::thread;

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
```
