use std::sync::mpsc;
use std::thread;
fn main() {
    // Create a channel and get a sender and receiver
    let (sender, receiver) = mpsc::channel();

    for number in vec![1, 2, 3, 4, 5] {
        // Clone the sender
        let thread_sender = sender.clone();
        // Spawn a thread which sends the number down the channel
        thread::spawn(move || {
            thread_sender.send(number).unwrap();
        });
    }

    // Drop the original sender
    // Program will hang without this line
    drop(sender);

    // Iterate over the receiver until all the senders have been dropped
    for msg in receiver {
        println!("Received message {:?}", msg);
    }
}