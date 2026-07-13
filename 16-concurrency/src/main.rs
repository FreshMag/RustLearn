use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // ____________________________________________________________________________________________
    // 1. First example, Spawning a Thread
    //---------------------------------------------------------------------------------------------
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("Iteration inside thread: {i}");
                thread::sleep(Duration::from_millis(10));
            }
        });

        for i in 1..5 {
            println!("Iteration inside MAIN thread {i}");
            thread::sleep(Duration::from_millis(10));
        }

        handle.join().unwrap();

        // How to use data from other threads?

        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's the vec: {v:?}")
        });

        // drop(v);  // thanks to us using the `move` keyword, the `v` vector has been moved into the
        // closure, protecting us from memory issues across threads by using ownership rules!

        handle.join().unwrap();
    }
    println!("-----------------------------------------------------------------------------------");
    // ____________________________________________________________________________________________
    // 2. Message passing
    //---------------------------------------------------------------------------------------------
    {
        // mpsc stands for "multiple producers, single consumer". This is that kind of channel
        // tx = transmitter
        // rx = receiver
        let (tx, rx) = mpsc::channel::<String>();

        thread::spawn(move || {
            let val = String::from("hello");
            // The `send` method of a Transmitter returns a Result<T, E>, so if the receiver has already
            // been dropped and there is no one to receive the message it will return an error
            tx.send(val)
        });

        // Receivers have two main methods:
        // - `recv()` : blocks and waits for messages. When one is available, returns a Result<T, E>
        //              containing Ok(T). When the transmitter closes, recv returns an Err.
        // - `try_recv()` : DOESN'T BLOCK. Returns a Result<T, E> immediately, an Ok containing a message
        //                  if one is immediately available, an Err if no message is currently available
        let received = rx.recv().unwrap();
        println!("Received: {received}");


        // Iterating over messages with multiple producers
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();

        thread::spawn(move || {
            let vals = vec![
                String::from("Hello"),
                String::from(" from"),
                String::from(" thread"),
                String::from("!")
            ];
            for v in vals {
                tx.send(v).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from(" more"),
                String::from(" messages"),
                String::from(" for"),
                String::from(" you!")
            ];
            for v in vals {
                tx1.send(v).unwrap();
            }
        });

        println!("Receiving");
        for v in rx {
            print!("{v}");
        }
    }
    println!();
    println!("-----------------------------------------------------------------------------------");
    // ____________________________________________________________________________________________
    // 3. Shared memory communication
    //---------------------------------------------------------------------------------------------
    {
        // Message passing is not the only way to communicate
        // Shared memory is another method, but it can be difficult to manage.
        // In a way, it's as Message Passing is single-ownership, while Shared Memory is multiple-ownership.
        // The second one is possible and powerful, but more difficult to manage.

        // Let's look at the Mutex<T> API
        let m = Mutex::new(5);

        {
            // LockResult (returned by `lock()`) would fail if another thread having the lock panicked.
            // So if another thread having the lock panicked in this example, the main would panic as well
            // NOTE: `lock()` blocks the current thread until it obtains the lock.
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("Value: {m:?}");
        // The call to lock returns a type called MutexGuard, wrapped in a LockResult that we
        // handled with the call to unwrap. The MutexGuard type implements Deref to point at our
        // inner data; the type also has a Drop implementation that releases the lock automatically
        // when a MutexGuard goes out of scope, which happens at the end of the inner scope. As a
        // result, we don’t risk forgetting to release the lock and blocking the mutex from being
        // used by other threads because the lock release happens automatically.


        // Now let's create a program that spawns 10 threads that collectively count up to 10.
        //
        // let counter = Mutex::new(0);
        // let mut handles = vec![];
        //
        // for _ in 0..10 {
        //     let handle = thread::spawn(move || {
        //         let mut num = counter.lock().unwrap();
        //
        //         *num += 1;
        //     });
        //     handles.push(handle);
        // }
        //
        // for handle in handles {
        //     handle.join().unwrap();
        // }
        //
        // println!("Result: {}", *counter.lock().unwrap());

        // This doesn't compile! The reason is we cannot move ownership of counter multiple times!
        // We could think about using a Rc<T> for managing multiple ownerships, but unfortunately
        // Rc<T> is not atomic, so it might mess up with the counting of references due to race conditions
        // Let's instead use Arc<T>, where the "A" stands for "Atomic"

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
        println!("Total count is: {:?}", *counter.lock().unwrap());
    }
    // ____________________________________________________________________________________________
    // 4. `Send` and `Sync` traits
    //---------------------------------------------------------------------------------------------

    // Very little of all of this concurrency model is part of the Rust language: it is in fact part
    // of the standard library, not the language.

    // Two traits are instead part of the language: Send and Sync.
    // They are "marker" traits, meaning that they don't require to implement methods.
    // They are implemented automatically, so implementing them manually is unsafe

    // The Send marker trait indicates that ownership of values of the type implementing Send can be
    // transferred between threads. Almost every type in Rust implements the `Send` trait.
    // Rc<T> is an exception, since passing its ownership between threads is not safe.

    // The Sync marker trait indicates that it is safe for the type implementing Sync to be
    // referenced from multiple threads. In other words, any type T implements Sync if &T
    // (an immutable reference to T) implements Send, meaning the reference can be sent safely to
    // another thread. Similar to Send, primitive types all implement Sync, and types composed
    // entirely of types that implement Sync also implement Sync.
    // The RefCell<T> type and the family of related Cell<T> types don’t implement Sync
}


