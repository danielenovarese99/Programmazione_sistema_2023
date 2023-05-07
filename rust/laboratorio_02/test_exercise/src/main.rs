use std::ops::Deref;

struct Samplestruct {
    values: Vec<i32>,
}
impl Samplestruct{
    fn new() -> Samplestruct{
        Samplestruct{
            values: vec![]
        }
    }

    fn print_values(&self) -> (){
        for i in 0..(*self).values.len(){
            println!("{}",(*self).values[i]);
        }
    }
    fn add_values(&mut self,newvalue : i32){
        (*self).values.push(newvalue);
    }
}
fn test_add(teststruct : &mut Samplestruct){
    (*teststruct).add_values(64);
}
fn main() {
    let mut s1 = Samplestruct::new();
    let test = &mut s1;
    (*test).values.push(1);
    s1.print_values();

    test_add(&mut s1);
    s1.print_values();



}
