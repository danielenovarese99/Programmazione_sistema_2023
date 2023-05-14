use std::io::{Read, Write};

/// buffer circolare
///
struct CircularBuffer<T>{
    max_size: i32,
    read_index: i32,
    write_index: i32,
    oldest_item: i32,
    buffer: Vec<Option<T>>
}

impl<T> CircularBuffer<T>{
    fn new(max_size: i32) -> Self{
        let mut vec = Vec::with_capacity(max_size as usize);
        for i in 0..max_size{vec.push(None)}

        CircularBuffer{
            max_size,
            read_index: 0,
            write_index: 0,
            oldest_item: 0,
            buffer: vec,
        }
    }

    /// USE PRINT STATEMENTS TO TEST FUNCTIONALITIES
    fn insertItem(&mut self, item: T){
        // insert element inside of buffer
        // first, check if buffer is full - write_index + 1 == read_index
        // if it isn't, insert in buffer at position write_index, then
            // if write_index + 1 == max_size, write_index = 0
                // if not, write_index = write_index + 1

        // check if buffer is full
        if self.write_index + 1 == self.read_index{
            println!("Buffer is full - please first remove an item.");
            return;
        }
        // if buffer is not full, insert item
        self.buffer[self.write_index as usize] = Some(item);
        println!("Item added succesfully");

        if self.write_index + 1 == self.max_size{
            self.write_index = 0;
        }
        else{
            self.write_index += 1;
        }
    }

    fn removeItem(&mut self)->Option<T>{
        // to remove an item, first retrieve the item at position read_index.
        // since it's an option, first check if it's an error - if it is, then simply keep trying to retrieve items until
        // a valid one is found or write_index is reached.

        // if it's valid, simply increase read_index by one and return that value.
        /// the current implementation tries looking for a valid value until either one is found or the write_index is reached
        /// ( stop when reaching write_index otherwise the loop never ends)
        let x = self.read_index.clone();
        if self.read_index == self.write_index{
            println!("Cannot read - no items are present.");
            return None;
        }

        if self.read_index + 1 == self.max_size{
            self.read_index = 0;
        }
        else{
            self.read_index += 1;
        }

        self.buffer.remove(x as usize)
    }

}

fn main() {
    println!("Hello, world!");
    let mut my_buffer: CircularBuffer<i32>= CircularBuffer::new(10);
    my_buffer.insertItem(1);
    my_buffer.insertItem(2);
    my_buffer.insertItem(3);
    let first_item = my_buffer.removeItem();
    if first_item.is_some(){
        println!("{}",first_item.unwrap());
    }
    else {
        println!("Error - first item is None");
    }

    println!("{:?}",my_buffer.buffer[0]);

}

/// the problem in this exercise is that when you remove an element from a vector, the vector is reallocated with size-1 -
/// we have to implement a way of removing items from this vector, and assigning a default value after removing them.
