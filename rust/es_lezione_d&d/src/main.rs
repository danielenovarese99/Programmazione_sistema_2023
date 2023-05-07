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
