use std::{thread};
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("привет число {} из порожденного потока!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("привет число {} из главного потока!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // Передача владения в поток
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Вот вектор {:?}", v);
    });
    handle.join().unwrap();

    // Создание каналов между потоками
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            "Hello".to_string(),
            "from".to_string(),
            "executing".to_string(),
            "thread".to_string(),
        ];
        for val in vals  {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });
    thread::spawn(move || {
        let vals = vec![
            "Another".to_string(),
            "good".to_string(),
            "message".to_string(),
            "for".to_string(),
            "you".to_string(),
        ];
        for val in vals  {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });
    // let receiver = rx.recv().unwrap();
    for received in rx {
        println!("We get: {}", received)
    }

    // Совместное использование ресурса, одноразовое через Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    };
    for handle in handles {
        handle.join().unwrap();
    };
    println!("Result is: {}", *counter.lock().unwrap());

    // Маркерные типажи Send и Synch.
    
}
// 427
