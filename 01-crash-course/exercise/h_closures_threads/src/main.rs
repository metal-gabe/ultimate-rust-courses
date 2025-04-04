// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
// use tokio::sync::broadcast;

use h_closures_threads::{expensive_sum, pause_ms};

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // 2. Spawn a child thread and have it call `expensive_sum(my_vector)`.  Store the returned
    // join handle in a variable called `handle`. Once you've done this you should be able to run
    // the code and see the Child thread output in the middle of the main thread's letters
    //
    let handle = thread::spawn(move || {
        println!("Child thread: calculating expensive sum");
        expensive_sum(my_vector)
    });

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {letter}");
        pause_ms(200);
    }

    // 3. Let's retrieve the value returned by the child thread once it has exited.  Using the
    // `handle` variable you stored the join handle in earlier, call .join() to wait for the thread
    // to exit with a `Result<i32, Err>`.  Get the i32 out of the result and store it in a `sum`
    // variable.  Uncomment the println.  If you did 1a and 1b correctly, the sum should be 20.
    //
    let sum = handle.join().unwrap();
    println!("The child thread's expensive sum is {sum}");

    // Time for some fun with threads and channels!  Though there is a primitive type of channel
    // in the std::sync::mpsc module, I recommend always using channels from the crossbeam crate,
    // which is what we will use here.
    //
    // 4. Uncomment the block comment below (Find and remove the `/*` and `*/`).  Examine how the
    // flow of execution works.  Once you understand it, alter the values passed to the `pause_ms()`
    // calls so that both the "Thread B" outputs occur before the "Thread A" outputs.

    let (tx, rx) = channel::unbounded();
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread.
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(400);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    // println!("AHHHHHHH");
    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // closed.  Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("BLLR? -- Main thread: Received {msg}");
    }

    // Join the child threads for good hygiene.
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    // Challenge: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.

    /* ========================================================================== */
    // NOTE :: trying to use one transmitter (sending to one receiver & its clone)
    // NOTE :: to get the same message in both child threads does not work
    // see this GPT link for more info: https://chatgpt.com/share/674d3f5e-a4a4-8006-b7f8-8daf9e74f755
    //
    // the answer is to use separate channels for each child thread (implemented below)
    //
    // there was a mention of trying to use the `tokio` crate and its `broadcast` module (I do like this option)
    // but `tokio` is an async/await operation and causes issues when used in child threads
    /* ========================================================================== */
    let (tx_c, rx_c) = channel::unbounded();
    let (tx_d, rx_d) = channel::unbounded();

    let handle_c = thread::spawn(move || {
        for msg in rx_c {
            println!("Thread C: Received {msg}");
        }
    });

    let handle_d = thread::spawn(move || {
        for msg in rx_d {
            println!("Thread D: Received {msg}");
        }
    });

    for msg in vec!["bllr-a", "bllr-b", "bllr-c", "bllr-d", "bllr-e", "bllr-f"] {
        tx_c.send(msg).unwrap();
        tx_d.send(msg).unwrap();
    }

    drop(tx_c);
    drop(tx_d);
    handle_c.join().unwrap();
    handle_d.join().unwrap();
    println!("Main thread: Exiting.")
}
