use std::fs::read;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn read_value()->i32{
    return 6
}
fn producer(sensor_id: usize, tx: Sender<(usize,i32)>){
    while true{
        thread::sleep(Duration::from_secs(5));
        println!("Sending values");
        tx.send((sensor_id,read_value())).unwrap()
    }
}


fn consumer(consumer_id: usize, rx: Receiver<(usize,i32)>){
    let mut sensor_count = 0;
    let mut current_speed = 0;
    let mut new_val;
    while true{
        new_val = rx.recv();
        if new_val.is_ok(){
            sensor_count += 1;
            current_speed += new_val.unwrap().1;
            if sensor_count < 10{
                continue;
            }
            else{
                sensor_count = 0;
                if current_speed > 50{
                    println!("Slow down there cowboy > {}", current_speed / 2);
                    current_speed = current_speed / 2;
                }
                else{
                    println!("Lets speed things up > {}",current_speed + 10);
                    current_speed = current_speed + 10;
                }

            }
        }
    }
}



fn main(){
    println!("Producers and consumer!");
    // create 10 producers, and 1 consumer.
    //let (tx,rx) = bounded::<(usize,i32)>(10);
    let (tx,rx) = channel();
    let mut thread_vec = Vec::new();
    for i in 0..10{
        let new_tx = tx.clone();
        thread_vec.push(
            thread::spawn(move || {
                    producer(i,new_tx);
            })
        )
    }
    thread_vec.push(thread::spawn(move || {
        consumer(0,rx);
    }));

    for x in thread_vec{
        x.join().unwrap();
    }

}