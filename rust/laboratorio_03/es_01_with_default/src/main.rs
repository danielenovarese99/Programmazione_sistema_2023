use std::cmp::max;
use std::mem;
use std::mem::take;

struct CircularBuffer<T>{
    max_index: i32,
    read_index: i32,
    write_index: i32,
    buffer: Vec<T>
}
/// the implementation of T requires that T has a default method

impl<T> CircularBuffer<T> where T: Default{
    fn new(max_index: i32)->Self{
        let mut vec = Vec::with_capacity(max_index as usize);
        for i in 0..max_index{vec.push(T::default())}
        CircularBuffer{
            max_index,
            read_index: 0,
            write_index: 0,
            buffer: vec,
        }
    }

    fn insert_item(&mut self,item: T){
        if self.write_index + 1 == self.read_index || (self.write_index + 1 == self.max_index) && self.read_index == 0{
            println!("Buffer is full - please first remove an item");
            return;
        }

        self.buffer[self.write_index as usize] = item;
        if self.write_index + 1 == self.max_index{
            self.write_index = 0;
        }
        else{
            self.write_index += 1;
        }

        println!("Item inserted succesfully");
    }

    fn remove_item(&mut self)->T{
        let x = self.read_index.clone();
        if self.read_index == self.write_index{
            println!("Cannot read - no items are present");
            return T::default();
        }

        if self.read_index + 1 == self.max_index{
            self.read_index = 0;
        }else{
            self.read_index += 1;
        }

        mem::take(&mut self.buffer[x as usize])
    }
}
fn main() {
    println!("Hello, world!");
    let mut circular_buffer1: CircularBuffer<String> = CircularBuffer::new(10);
    circular_buffer1.insert_item(String::from("ciao"));
    circular_buffer1.insert_item(String::from("sono"));
    circular_buffer1.insert_item(String::from("daniele"));
    let x1 = circular_buffer1.remove_item();
    let x2 = circular_buffer1.remove_item();
    let x3 = circular_buffer1.remove_item();
    let x4 = circular_buffer1.remove_item();

    println!("{:?}",circular_buffer1.write_index);
    println!("Item 1 > {}",x1);
    println!("Item 2 > {}",x2);
    println!("Item 3 > {}",x3);
    println!("Item 4 > {}",x4);


}
