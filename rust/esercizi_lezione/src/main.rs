
/*
extern crate core;

mod dandd;

use crate::dandd::characters::{DDCharacter, DDClasses};
use crate::dandd::print_my_character;

fn main() {
    let my_warrior = /* match*/ DDCharacter::<f32>::new(1).unwrap(); /* {
        Some(d) => d,
        None => DDCharacter::<f32>::default_wizard() //panic!("Aiuto")
    };*/

    let is_ok_my_w = DDCharacter::<f32>::new(0)
        .unwrap_or(DDCharacter::<f32>::default_elf());
    /* if is_ok_my_w.is_none() {
         panic!("Aiuto");
     }*/
    println!("My warrior: {}", my_warrior.name);
    print_my_character(my_warrior);

    let my_wizard = DDCharacter::<u8>::default_wizard();
    print_my_character(my_wizard);
}

 */
fn makeBox(a : i32) ->Box<(i32,i32)> {
    let b = Box::new((a,1));
    return b;
}
fn printBox(a: Box<(i32,i32)>) -> (){ // example of how a function takes parameters and afte borrowing them, frees them.
    // this results in losing permanently the values we have passed it
    println!("The box is ({}, {})",(*a).0,(*a).1);
}
fn main() {
    println!("Hello world");

    // Box::new(val) lets us assign items on the heap manually.
    /*
    let mybox = makeBox(7);
    println!("{:?}",(*mybox));
    let x = (*mybox).0 + 3;

    println!("{} = {} + 3",x,(*mybox).0);
    printBox(mybox); // once you pass it, it's gone
    println!("My box should be gone - let's test it {:?}",mybox);
     this causes an error -> mybox has been passed to the printBox function, and has now been lost forever.
     */



    // Arrays
    /*
    let mut test_array: [i32;5] = [1,2,3,4,5];
    println!("My array => {:?};\nLength is {}",test_array,test_array.len());
   /// same borrow / move principles apply to array items
   let array_copy = &test_array[0..=2]; // can copy arrays by slicing or by whole by using
    // let array_copy: &[i32] = &test_array; // copies entire array
    println!("{:?}",array_copy);
     */


    //     Vettori allocati dinamicamente su Heap -- Vec
    /*
    let mut new_vettore: Vec<i32> = Vec::new();
    new_vettore.push(1);
    new_vettore.push(2);
    new_vettore.push(3);
    let s = &mut new_vettore; // s contains all ALIVE elements of new_vettore at the moment of assignment
    // cannot push new elements though (its like a slice)
    println!("{:?}",s);

     */



    // Stringhe
    /*
    statiche (str) e dinamiche (String)

    strings assigned by direct assignment such as
    let a = "ciao"; are static strings
    To convert them into dynamic ones and be able to manipulate them, we need to use the to_string() method.
    This creates a buffer on the heap with a pointer to it, a maxlength and current length; (just as a Vec<T>)

    str (static) methods are also usable on Strings, but not viceversa.

    With String (dynamic) come all the normal string methods, such as to_upperCase(), trim(), replace().... you experiment
    Just remember that you can only manipulate dynamic ones, so always turn &str to String or use proper methods such as
    String::from_text("...") or "....".to_String()

    let a: &str = "ciao";
    let mut b: String = String::new();
    b.push_str(a);
    b.push_str(" daniele");

    println!("{}",b);
    println!("{}","  come stai?   ".to_string().trim()); /// creating dynamic string from static one then removing spaces and printing it
     */

    // FUNZIONI
    /*
    Se non hanno return value
    fn function(..){}

    sennò
    fn function(..) -> return value{..}

let x = 2;

    fn print_number(a:i32) /* -> () */ {
        println!("{}",a);
    }
    print_number(x);


    fn add_numbers(a:i32, b:i32) -> i32 { // the return value is an i32
        a+b // the returned value doesn't need to have ";"
    }


    let a = 10;
    let b = 20;
    let c = add_numbers(a,b);
    println!("{}",c);

    fn print_up_to_number(number: i32){ // implementing a simple loop
        let mut count = 0;
        // can use 'label: loop
        // to label loops -> can break out of any loop at any point using
        // break 'label;
        loop{
            count = count + 1;
            println!("{}",count);
            if count == number { break; };
        }
        println!("\n\n");
    }
    print_up_to_number(15);

    fn for_loop_example(n: [i32;5]){ // implementing a for loop using an iterable (for x in array / slice {...}
        for x in n{
            println!("{}",x);
        }
    }
    let n: [i32;5] = [1,2,3,4,5];
    for_loop_example(n);

    let names = ["Bob","Frank","Willy"];
    for x in names{ // what is the difference between using x in names AND x in names.iter() ??
        println!("{}",x);
    }
     */

    // Match
    /*

    match must contain ALL POSSIBLE CASES

    _=> is used to handle all the default cases (all except described ones)
    value => { handling case....}

    let mut count = 0;
        loop{
            count = count + 1;
            if count == 30 {break};
            match count{
                0 => println!( "zero"),
                1..=10 => println!("tra uno e dieci"),
                11..=20 => println!("tra undici e venti"),
                21..=30 => println!("buon voto"),
                _=> println!("voto non valido")
            };
        }
     */


}

