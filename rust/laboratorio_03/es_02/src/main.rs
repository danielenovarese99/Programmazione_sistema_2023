/*
enum I<'a>{
    Int(&'a dyn Iterator<Item=(i32)>),
    Str(&'a dyn Iterator<Item=(String)>),
    StaticStr(&'a dyn Iterator<Item=(&'a str)>),
}

enum I{
    MyCycleItem(MyCycle<T>),
    GenericItem(T),
}

 */

enum PossibleIterators<I> {
    Cycle(MyCycle<I>),
    NormalIter(I),
}
struct MyCycle<I: Clone+Iterator>{
    iter: I,
    iter_clone: I,
    repeat: usize,
    current_repeat: usize,
}

struct Cycle<I: Clone + Iterator>{
    iter: I,
    iter_clone: I,
    repeat: usize,
    current_repeat: usize,
}
/// I is an iterator >> you can't return Option<I> but you have to return something of type contained inside of I
impl <I> MyCycle<I> where I: Clone + Iterator{

    fn new(item: I, repeat: usize) -> Self{
        let iter_clone = item.clone();
        MyCycle{
            iter: item,
            iter_clone,
            repeat,
            current_repeat: 0
        }
    }


      fn next(&mut self)->Option<I::Item>{
        let next_item = self.iter.next();
          // if i still have items, return the next item
        if next_item.is_some(){
            return next_item;
        }
        else{
            // if i dont, then check for the total amount of repetitions i need
            if self.repeat == 0{ /// if it's 0, then infinite loop - clone starting iter and keep going
                let new_iter = self.iter_clone.clone();
                self.iter = new_iter;
                return self.iter.next();
            }
            else{
                // if it's not 0, then check the current counter
                if self.current_repeat == self.repeat{
                    // if the current counter = max repetitions return None - the iterator has finished.
                    return None;
                }else{
                    // if the current counter hasn't reached the end of the repetitions needed, add 1 to it and restart it.
                    self.current_repeat += 1;
                    let new_iter = self.iter_clone.clone();
                    self.iter = new_iter;
                    return self.iter.next();
                }
            }
        }
    }

}
fn main() {
    println!("Hello, world!");
    let my_test = vec![1,2];
    //let mut c = my_test.iter();
    let mut c = my_test.iter();
    println!("{:?}",c.next());
    println!("{:?}",c.next());
    println!("{:?}",c.next());

    /// TESTING REPEAT
    let mut test_mycycle = MyCycle::new(0..2, 0);
    println!("{:?}, repeat = {:?}",test_mycycle.next(),test_mycycle.current_repeat);
    println!("{:?}, repeat = {:?}",test_mycycle.next(),test_mycycle.current_repeat);
    println!("{:?}, repeat = {:?}",test_mycycle.next(),test_mycycle.current_repeat);

    /// TESTING EMPTY
    let empty_iter: Vec<i32> = Vec::new();
    let mut empty_iter2 = empty_iter.iter();
    let mut test_empty_mycycle = MyCycle::new(empty_iter2,0);

    println!("{:?}, repeat = {:?}",test_empty_mycycle.next(),test_mycycle.current_repeat);


   /*
    /// TESTING MYCYCLE MOLTIPLICATION
    //let mut c = MyCycle::new(MyCycle::new(0..2, 2), 3);
    println!("Total repetions : {:?} > should be 25",c.repeat);
    */
}
