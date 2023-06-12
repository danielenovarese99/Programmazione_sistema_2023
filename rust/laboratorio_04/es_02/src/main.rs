use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Seek, Write};
use std::{mem, thread};
use std::fmt::{Debug, Formatter};
use std::mem::size_of;
use std::ops::Deref;
use std::ptr::write;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};
use std::thread::sleep;


#[repr(C)]
#[derive(Clone,Copy)]
struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32
}
impl SensorData{
    /// AVG, MAX, MIN
    fn avg(&self) -> f32{
        let mut tot: f32 = 0.0;
        self.values.map(|e| tot = tot + e);
        tot / self.values.len() as f32
    }
    fn max(&self) -> f32{
        let mut current_max: f32 = 0.0;
        for i in 0..self.values.len(){
            if self.values[i] > current_max{
                current_max = self.values[i].clone();
            }
        }
        current_max
    }
    fn min(&self) -> f32{
        let mut current_min: f32 = self.values[0].clone();
        for i in 0..self.values.len(){
            if self.values[i] < current_min{
                current_min = self.values[i].clone();
            }
        }
        current_min
    }
}

struct DataBuffer{
    max_size: u32,
    read_index: u32,
    write_index: u32,
    data: Vec<SensorData>
}
impl DataBuffer{
    fn new(max_size: u32)->Self{
        DataBuffer{
            max_size,
            read_index: 0,
            write_index: 0,
            data: Vec::new()
        }
    }


    fn db_write(&mut self,mydata: Vec<SensorData>){
        println!("Writing items >> ");
        for i in 0..mydata.len() {
            if self.write_index != 0 && self.read_index != 0 { // consider starting condition where both indexes are at 0
                if self.write_index + 1 == self.read_index || (self.write_index + 1 == self.max_size && self.read_index == 0) {
                    println!("Read is required - stopping write process...");
                    return;
                }
            }
            if self.write_index == self.max_size{
                if self.read_index == 0{
                    println!("Read required - ");
                    return;
                }
            }
            self.data.push(mydata[i]);
            if self.write_index >= self.max_size{
                // if end of buffer is reached, start from beginning > circular buffer
                self.write_index = 0;
            }
            else{
                self.write_index = self.write_index + 1;
            }
        }
        println!("max > {}, read > {}, write > {}", self.max_size, self.read_index, self.write_index)
    }
    fn db_read(&mut self){
        println!("Reading items >> ");
        /// read items
        while self.read_index != self.write_index{
            /// read data from buffer, size_of::<SensorData>
            /// use functions to compute avg, max, min
            /// update read_index accordingly to circular buffer rules



            let current_max = self.data[self.read_index as usize].max();
            let current_avg = self.data[self.read_index as usize].avg();
            let current_min = self.data[self.read_index as usize].min();
            println!("ITEM #{} >> AVG : {}, MAX : {}, MIN : {}",self.read_index,current_avg,current_max,current_min);

            /// update current index according to circular buffer rules
            if self.read_index == self.max_size{
                self.read_index = 0;
            }
            else{
                self.read_index = self.read_index + 1;
            }
        }
        println!("max > {}, read > {}, write > {}", self.max_size, self.read_index, self.write_index)
    }
}



fn main(){

    // create starting data
    let mut mydata: Vec<SensorData> = vec![];
    for i in 0..10{
        let mut myfloat_values = [0.0f32;10];
        for x in 0..10{
            myfloat_values[x] = 1.2;
        }
        let new_sensor_data = SensorData{
            seq: i,
            values: myfloat_values,
            timestamp: 50,
        };
        mydata.push(new_sensor_data);
    }

    let mut my_circular_buffer = DataBuffer::new(20);

    let copy_for_write = Arc::new(Mutex::new(my_circular_buffer));
    let copy_for_read = Arc::clone(&copy_for_write); // same address of other arc, strong pointer


    //let shared_file = Arc::new(Mutex::new(myfile));
    // ARC defines a shared variable between threads
    // mutex is used to lock and unlock


    let write_thread = thread::spawn(move || {
        /// each 3 seconds, try writing
        let interval = Duration::from_secs(3);
        let mut next_time = Instant::now() + interval;
        loop{
            let mut v = copy_for_write.lock().unwrap();
            v.db_write(mydata.clone());
            sleep(next_time - Instant::now());
            next_time += interval;
        }
    });
    let read_thread = thread::spawn(move || {
        /// each 3 seconds, try reading
        let interval = Duration::from_secs(3);
        let mut next_time = Instant::now() + interval;
        loop{
            let mut v = copy_for_read.lock().unwrap();
            if v.write_index != v.read_index{
                v.db_read();
            }
            sleep(next_time - Instant::now());
            next_time += interval;
        }
    });

    write_thread.join().unwrap();
    read_thread.join().unwrap();

}
