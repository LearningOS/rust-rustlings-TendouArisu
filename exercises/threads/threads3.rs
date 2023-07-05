// threads3.rs
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a hint.



use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Queue>, tx: Arc<Mutex<mpsc::Sender<u32>>>) -> () {
    let qc1 = Arc::clone(&q);
    let txnew = Arc::clone(&tx);

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            let temp = txnew.lock().unwrap();
            temp.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    let qc2 = Arc::clone(&q);
    let txnew = Arc::clone(&tx);
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            let temp = txnew.lock().unwrap();
            temp.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;
    let txnew = Arc::new(Mutex::new(tx));

    send_tx(Arc::new(queue), txnew);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
