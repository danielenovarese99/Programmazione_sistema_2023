pub mod characters;
use crate::dandd::characters::{DDCharacter, DDClasses};
use std::fmt::Display;

pub fn print_my_character<T: Display>(c: DDCharacter<T>) {
    print!("My character ({}) is: ", c.name);

    match c.character_class {
        DDClasses::WARRIOR => print!("a Warrior!!!"),
        DDClasses::WIZARD(w) => println!("a Wizard with ({})", w),
        DDClasses::ELF => print!("an Elf!"),
        DDClasses::THIEF => print!("a Thief..."),
        _ => {}
    }

    print!("\nPoints: {}", c.general_points);
    print!("\n");
}