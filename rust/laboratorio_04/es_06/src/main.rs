use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct SpeedCounter{
    current_speed: i32,
    sensor_count: i32,
    used_data: i32,

}

struct CyclicBarrier{
    speedcounter: Mutex<SpeedCounter>,
    n_sensors: i32,
    cond: Condvar,
}

pub fn read_value()->i32{
    println!("Reading value");
    return 7
}

impl CyclicBarrier{
    pub fn new(sensors: i32)->Self{
        CyclicBarrier{
            speedcounter: Mutex::new(SpeedCounter{current_speed: 0, sensor_count: 0,used_data: 0}),
            n_sensors: sensors,
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self,new_speed: i32){
        let mut current_lock = self.speedcounter.lock().unwrap();
        if current_lock.sensor_count == 0{
            current_lock.used_data = 0;
        }
        current_lock.sensor_count += 1;
        current_lock.current_speed += new_speed;
        if current_lock.sensor_count < self.n_sensors{
            // lock the process
            current_lock = self.cond.wait(current_lock).unwrap();
        }
        else{
            current_lock.sensor_count = 0;
            self.cond.notify_all();
        }
        return;
    }

    pub fn set_speed(&self){
        let mut lock = self.speedcounter.lock().unwrap();
        if lock.used_data == 1{
            return;
        }
        else{
            if lock.current_speed > 50{
                println!("Screeech - slow down there cowboy!");
                lock.current_speed -= 20;
                lock.used_data = 1;
                return;
            }
            else{
                println!("Come on! We ain't got time to waste");
                lock.current_speed += 20;
                lock.used_data = 1;
                return;
            }
        }
    }



}
fn main() {
    println!("Hello, world!");
    // create threads and start them up
    let mut thread_vec = Vec::new();
    let cyclicbarrier = Arc::new(CyclicBarrier::new(10));
    for i in 0..10{
        let copy_arc = cyclicbarrier.clone();
        thread_vec.push(
            std::thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(3));
                    let current_speed = read_value(); // read random value
                    copy_arc.wait(current_speed); // wait and update speed counter
                    copy_arc.set_speed();
                }
            })
        )
    }
    for mythread in thread_vec{
        mythread.join().unwrap();
    }
}
