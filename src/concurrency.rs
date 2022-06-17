use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub fn run(){
    let (olly_tx, olly_rx) = mpsc::channel();
    let (sam_tx, sam_rx) = mpsc::channel();

    let olly_handle = thread::spawn(move || {
        olly_chat(sam_tx, olly_rx);
    });

    let sam_handle = thread::spawn(move || {
        sam_chat(olly_tx, sam_rx);
    });

    match olly_handle.join(){
        Ok(_) => {}
        Err(_) => {}
    }
    match sam_handle.join(){
        Ok(_) => {}
        Err(_) => {}
    }

}

fn sam_chat(olly_tx:Sender<&str>, sam_rx: Receiver<&str>){
    let result = sam_rx.recv();
    println!("{}", result.unwrap());
    let _send_result = olly_tx.send("Hello Olly.");
}

fn olly_chat(sam_tx:Sender<&str>, olly_rx: Receiver<&str>){
    let _send_result = sam_tx.send("Hello Sam.");
    let result = olly_rx.recv();
    println!("{}", result.unwrap());
}