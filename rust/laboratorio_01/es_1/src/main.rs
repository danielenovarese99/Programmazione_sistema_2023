use std::env::args;

fn capitalize_in_place(s: &str) -> String{
    let mut temp_string = String::from(s);
    let mut temp_char = temp_string.remove(0);
    temp_char = temp_char.to_uppercase().to_string().remove(0);
    temp_string.insert(0,temp_char);
    temp_string.push_str(" ");
    temp_string
}
fn main() {
    let mut args: Vec<String> = args().collect(); // TODO >> IMPLEMENT IN-PLACE VERSION (IF IT EXISTS)
    if args.len() > 1 {
        for mut each in args {
            each = capitalize_in_place(each.as_str());
            print!("{}", each);
        }
    }
}
/*
How did it work?
All i did was retrieve the arguments into an array of dynamic strings - then passed each one of them to a function, that altered it.
Still need to check wether or not this operation is considered "in place"...
 */
