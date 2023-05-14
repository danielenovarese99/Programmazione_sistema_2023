use std::ops::Deref;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc;


struct Person {
    name : String,
    age: RefCell<i32>,
}

impl Person{
    fn print_age(&self){
        let mut x = self.age.borrow_mut();
        println!("My name is {} and i'm {} years old. Yay!", self.name,x);
    }

    fn birthday(&self){
        println!("Today is my birthday! +1 up");
        let mut x = self.age.borrow_mut();
        *x += 1;
    }
}

fn main() {
    let mut daniele = Person{
        name : String::from("Daniele"),
        age : 23,
    };


}
