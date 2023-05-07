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
    fn avg(self) -> f32{
        let mut tot: f32 = 0.0;
        self.values.map(|e| tot = tot + e);
        tot / self.values.len() as f32
    }
    fn max(self) -> f32{
        let mut current_max: f32 = 0.0;
        for i in 0..self.values.len(){
            if self.values[i] > current_max{
                current_max = self.values[i].clone();
            }
        }
        current_max
    }
    fn min(self) -> f32{
        let mut current_min: f32 = self.values[0].clone();
        for i in 0..self.values.len(){
            if self.values[i] < current_min{
                current_min = self.values[i].clone();
            }
        }
        current_min
    }
}

/// CIRCULAR BUFFER DATA
/// header >>>>>
/// max_capacity
/// write_index
/// read_index

unsafe fn file_write(mut myfile: &MutexGuard<File>, mydata: Vec<SensorData>) -> Result<()>{
    /// PERSONAL COMMENT
    /// this is kinda tricky - cause in the occasion where two writes are called after eachother, we would have to first check if the
    /// write and read indexes differ by one - if they do, we can simply return Ok(()) - which would make the write thread stop
    /// until a read thread is called, therefore causing no problems.
    /// if we don't, we would either have to add the new data to the end of the file (appending to current file)
    /// or we would have to first read all the data present - save it in a temporary buffer, then add to that
    /// temporary buffer all the new data, and re-write it completely
    /// this would have to be done in order to maintain coherence with all the code inside of this file, which is
    /// based off the idea that the entire file is re-written everytime. (not very efficient)
    /// SO >>>>>
    /// The solution i adapted here is to simply return if the write-index doesn't differ by one by the read index.
    /// read indexes
    let mut indexes_buffer = [0u8;3];
    myfile.deref().read_exact(&mut indexes_buffer)?;

    let max_size = u8::from_le_bytes(indexes_buffer[0..1].try_into().unwrap());
    let read_index = u8::from_le_bytes(indexes_buffer[1..2].try_into().unwrap());
    let mut write_index = u8::from_le_bytes(indexes_buffer[2..3].try_into().unwrap());


    /// try writing all elements
    let mut temp_buffer: Vec<&[u8]> = vec![];
    for i in 0..10{
        /// always check indexes
        /// if indexes match a condition that stops writing, release lock
        if write_index != 0 && read_index != 0{ // consider starting condition where both indexes are at 0
            if write_index + 1 == read_index || (write_index + 1 == max_size && read_index == 0) {
                if temp_buffer.is_empty(){
                    println!("Read is required - stopping write process...");
                    return Ok(());
                }
                else{
                    break;
                }
            }
        }
        temp_buffer.push(from_struct_to_u8(&mydata[i]));
        // update write_index
        if write_index >= max_size{
            // if end of buffer is reached, start from beginning > circular buffer
            write_index = 0;
        }
        else{
            write_index = write_index + 1;
        }
    }
    /// eliminate all data from file
    /// write all new indexes + data
    myfile.deref().set_len(0).unwrap();
    myfile.deref().rewind().unwrap();

    myfile.deref().write(&[max_size,read_index,write_index])?;

    for i in 0..temp_buffer.len(){
        myfile.deref().write(temp_buffer[i])?;
    }
    ///

    println!("max > {}, read > {}, write > {}",max_size,read_index, write_index);
    Ok(())
}

fn file_read(mut myfile: &MutexGuard<File>) -> Result<()>{

    let mut indexes_buffer = [0u8;3];
    myfile.deref().read_exact(&mut indexes_buffer)?;

    let max_size = u8::from_le_bytes(indexes_buffer[0..1].try_into().unwrap());
    let mut read_index = u8::from_le_bytes(indexes_buffer[1..2].try_into().unwrap());
    let write_index = u8::from_le_bytes(indexes_buffer[2..3].try_into().unwrap());

    if read_index == write_index{
        println!("max > {}, read > {}, write > {}",max_size,read_index,write_index);
        return Ok(());
    }

    /// read items
    while read_index != write_index{
        /// read data from buffer, size_of::<SensorData>
        /// use functions to compute avg, max, min
        /// update read_index accordingly to circular buffer rules


        let mut temp_buffer = [0u8;size_of::<SensorData>()];
        myfile.deref().read_exact(&mut temp_buffer)?;

        /// de-serialize current struct
        let myseq = u32::from_le_bytes(temp_buffer[0..4].try_into().unwrap());
        let mut myvalues = [0.0f32;10];
        for i in 0..10{
            myvalues[i] = f32::from_le_bytes(temp_buffer[4+(4*i)..8+(4*i)].try_into().unwrap());
        }
        let mytimestamp = u32::from_le_bytes(temp_buffer[44..48].try_into().unwrap());

        let current_sensor_data = SensorData{
            seq: myseq,
            values: myvalues,
            timestamp: mytimestamp
        };
        println!("ITEM #{} >> AVG : {}, MAX : {}, MIN : {}",current_sensor_data.seq,current_sensor_data.clone().avg(),current_sensor_data.clone().max(),current_sensor_data.clone().min());

        /// update current index according to circular buffer rules
        if read_index == max_size{
            read_index = 0;
        }
        else{
            read_index = read_index + 1;
        }
    }

    // once the main loop is done, delete the entire content of the file and re-write the new indexes.

    myfile.deref().set_len(0).unwrap();
    myfile.deref().rewind().unwrap();
    /// update indexes
    myfile.deref().write(&[max_size,read_index,write_index])?;

    println!("max > {}, read > {}, write > {}",max_size,read_index,write_index);

    Ok(())
}


fn file_t_write(file: MutexGuard<File>, mydata: Vec<SensorData>){
    unsafe {
        match file_write(&file, mydata.clone()){
            Ok(..) => println!("Succesful write"),
            Err(e) => println!("Unsuccesful write: {:?}",e),
        };
    }
    file.deref().rewind().unwrap();
}
fn file_t_read(file: MutexGuard<File>){
    match file_read(&file){
        Ok(..) => println!("Succesful read"),
        Err(e) => println!("Unsuccesful read: {:?}",e),
    }
    file.deref().rewind().unwrap();
}

// (1) give input data to producer, which writes on file + adds to circular buffer
    // each time an item is written on the file and added on the buffer, if it's full, start read
// (2) after 10 seconds / when buffer is full, start read
// consumer reads from

// define two executables inside same file

// FILE STRUCTURE
// indice lettura - indice scrittura - dimensione max

// (1) producer >> opens file, writes 10 elements / until it reaches read_index
// closes file, calls consumer()

// (2) consumer >> opens file, reads 1 struct and updates read_index >>
//          if its at write_index, then stop and calll producer
//
unsafe fn from_struct_to_u8<T:Sized>(p: &T) -> &[u8]{
    /// transform any data type into the correspondent amount of u8 (bites), to write and read from file.
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}
fn main() -> std::io::Result<()>{

    // open main file
    let mut myfile: File = File::options().read(true).write(true).open("data.bin")?;
    myfile.set_len(0).unwrap();
    myfile.write(&[10,0,0,0])?; /// write max dimension and starting indexes
    myfile.rewind().unwrap();


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

    /*
     // create starting file
    let mut my_file = File::create("data.bin")?;
    my_file.write(&[15,0,0])?; /// write max dimension and starting indexes
    unsafe {
        for i in 0..mydata.len() {
            let my_u8: &[u8] = from_struct_to_u8(&mydata[i]);
            my_file.write(my_u8)?;
        }
    }
     */
    /*
    for i in 0..5{
        file_t_write(&myfile,mydata.clone());
        file_t_read(&myfile);
    }
     */
    /// the shared file between threads is our file
    let shared_file = Arc::new(Mutex::new(myfile));
    let copy_shared_file = Arc::clone(&shared_file);

    //let shared_file = Arc::new(Mutex::new(myfile));
    // ARC defines a shared variable between threads
    // mutex is used to lock and unlock


    let write_thread = thread::spawn(move || {
        /// each 3 seconds, try writing
        let interval = Duration::from_secs(3);
        let mut next_time = Instant::now() + interval;
       loop{
           let mut v = copy_shared_file.lock().unwrap();
           file_t_write(v,mydata.clone());
           sleep(next_time - Instant::now());
           next_time += interval;
       }
    });
    let read_thread = thread::spawn(move || {
        /// each 3 seconds, try reading
        let interval = Duration::from_secs(3);
        let mut next_time = Instant::now() + interval;
       loop{
           let mut v = shared_file.lock().unwrap();
           file_t_read(v);
           sleep(next_time - Instant::now());
           next_time += interval;
       }
    });

    write_thread.join().unwrap();
    read_thread.join().unwrap();



    Ok(())
}
