use std::env::args;
use crate::lib::{is_valid};

mod lib;



fn main(){
    let args: Vec<String> = args().skip(1).collect();
    if args.len() > 1{
        //println!("{:?}",args);
        let args_joined = args.join(" ");
        if args_joined.len() == 19 && args_joined.chars().filter(|x| (*x).is_digit(32)).count() == 16{
            if is_valid(args_joined.as_str()) == true{
                println!("Valid code");
            }
            else { println!("invalid code"); }

        }
        else{
            println!("Invalid code");
        }
        /*
        println!("{}",args_joined.len());
        if is_valid(args_joined.as_str()){
            println!("Code is valid");
        }
        else { println!("Invalid code"); }
         */

    }
    else{
        println!("No args!");
    }
}