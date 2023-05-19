use std::iter::{Chain, Cycle, Zip};


/*
Was the exercise that hard?
It wasn't.
What took me so long was realizing how iterators actually worked in Rust, and how Traits also worked.
This slowed the entire process down, as I didn't know what the problem was really asking me.

What it wanted was for me to implement a Cyclic iterator, that repeated the main iterator x times
If x was 0, then it would repeat it endlessly.

After understanding the problem (which took way longer than needed) - i tried constructing a struct with a few fields, and tried creating an
iterator that had as content the starting content * x times.This dindn't work.
Thanks to @Giovanni and his kindness, i tried using an enum.
I still had not completely understood the assignment, so it wasn't really helpful in the beginning...then i started to get it a little bit.

I implemented the main struct with a generic parameter I, being the iterator contained in the struct.
The struct is based off a main content, which is an enum - used to handle the two different cases, where repeat = 0, and repeat > 0.
The enum is really important as it makes it possible to handle the two different cases - from a single struct being MyCycle.

This means that calling MyCycle.next() will evaluate the right call to .next(), depending if its content is repeat = 0 or repeat > 0.
(i'm realizing now that perhaps i didn't need an enum at all)

All that was next was to create the next() function - which is a simple call to the I.next() function - being I an Iterator+Clone, it supports it.
In this function all we need to do is check if the I.next() value is None, or some value.
If it's some, return it (always the case of infinite enum case)
If it's none, and the current repetitions is smaller than the amount of times needed, re-copy the iterator into the main content and call I.next() again.

Realizing now that we didn't need the enum at all.
All i could've simply done was handle the I.next() call, and in case it returned None,
if repeat == 0 then simply recopy the main iterator in its content field - and call next() again
if repeat > 0, check if current_repeat was smaller than the required amount - if it was, copy, if it wasn't, return None.

This exercise was actually, really easy.
I completely missed the theory part, and that completely messed me up.
Completely.
 */
#[derive(Clone)]
struct MyCycle<I: Clone+Iterator>{
    content: PossibleIterators<I>,
    repeat: usize,
    content_copy: I,
    current_repeat: usize,
}
/// the goal is to create two possible iterators,
/// one that has an implementation with next() that repeats endelessly -
/// one that simply returns next() but has the original iterator repeated n times (flatten method)
/// one...
#[derive(Clone)]
enum PossibleIterators<I: Clone+Iterator>{
    InfiniteIter(Cycle<I>),
    FiniteIter(I),
}
impl <I> PossibleIterators<I> where I: Clone + Iterator {
    fn is_infinite(&self)->bool{
        match self{
            PossibleIterators::InfiniteIter(E) => {true},
            PossibleIterators::FiniteIter(E) => {false},
        }
    }
}

impl <I>Iterator for PossibleIterators<I> where I: Clone + Iterator{
    type Item = I::Item;


    fn next(&mut self) -> Option<Self::Item> {
        match self{
            PossibleIterators::InfiniteIter(E) => {
                let temp = E.next();
                if temp.is_some(){
                    return temp;
                }
                return None;
            },
            PossibleIterators::FiniteIter(E) => {
                let temp = E.next();
                if temp.is_some(){
                    return temp;
                }
                return None;
            },
        }
    }
}


impl <I> MyCycle<I> where I : Clone + Iterator{
    fn new(iter: I, repeat: usize) -> Self{
        if repeat == 0{
            let cloned_iter = iter.clone();
            let repeated_iter = iter.cycle();
            return MyCycle{content: PossibleIterators::InfiniteIter(repeated_iter),repeat,content_copy: cloned_iter,current_repeat: 0}
        }
        else{
            let iter_copy = iter.clone();
            return MyCycle{content: PossibleIterators::FiniteIter(iter_copy),repeat,content_copy: iter.clone(),current_repeat: 0}
        }
    }

    /*
    fn my_chain(&mut self, other: &mut MyCycle<I>) -> Vec<I::Item>{
        /// generate two iterators of length content * repeat, then chain them.
        /// iterator1 * repeat1      +       iterator2 * repeat2
        let mut iterator1 = self.content_copy.clone();
        let iterator2 = other.content_copy.clone();
        let repeat1 = self.repeat.clone();
        let repeat2 = other.repeat.clone();

        let mut final_result: Vec<I::Item> = Vec::new();
        let mut first_iterator_item = self.next();
        while first_iterator_item.is_some(){
            final_result.push(first_iterator_item.unwrap());
            first_iterator_item = self.next();
        }

        let mut second_iterator_item = other.next();
        while second_iterator_item.is_some(){
            final_result.push(second_iterator_item.unwrap());
            second_iterator_item = other.next();
        }

        return final_result;

    }
     */


}

impl <I>Iterator for MyCycle<I> where I: Clone + Iterator {
    type Item = I::Item;


    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.content.next();
        if return_value.is_some(){
            return return_value;
        }else{
            if self.current_repeat < self.repeat{
                let iterator_copy = self.content_copy.clone();
                self.content = PossibleIterators::FiniteIter(iterator_copy);
                self.current_repeat += 1;
                return self.content.next();
            }
            else{
                println!("Repetitions have finished for the given iterator.");
                return None;
            }
        }
        /*
        OLD FUNCTION


        THE NEW FUNCTION JUST CALLS .NEXT() ON ITS CONTENT
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
         */
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


    /// first text - if *repeat* == 0 repeat forever,
    /// if not, repeat only *x* amount of times

    // repeat infinite times
    let mut test_infinite_times = MyCycle::new(0..3, 0);
    println!("{:?} repeat infinite times", test_infinite_times.next());
    println!("{:?} repeat infinite times", test_infinite_times.next());
    println!("{:?} repeat infinite times", test_infinite_times.next());
    println!("{:?} repeat infinite times", test_infinite_times.next());

    // repeat x amount of times
    let mut test_repeat_x_times = MyCycle::new(0..2,1);
    //let mut test2 = MyCycle::new(MyCycle::new(0..3,2),0); - WORKS
    println!("{:?} repeat 1 time", test_repeat_x_times.next());
    println!("{:?} repeat 1 time", test_repeat_x_times.next());
    println!("{:?} repeat 1 time", test_repeat_x_times.next());
    println!("{:?} repeat 1 time", test_repeat_x_times.next());
    // println!("{:?} repeat 1 time", test_repeat_x_times.next());
    println!("SPACE SPACE SPACE \n\n\n");


    // build mycycle from existing mycycle
    // how does this work? When calling next, it calls .next() on its content >> .next() is called from the inner mycycle,
    // that is executed first - when the inner is finished iterating over its content, it will start executing the one at the top, but with the same content.
    let mut test_from_mycycle = MyCycle::new(MyCycle::new(0..1,1),1);
    println!("{:?}",test_from_mycycle.next());
    println!("{:?}",test_from_mycycle.next());
    println!("{:?}",test_from_mycycle.next());
    println!("{:?}",test_from_mycycle.next());



    /// test chain function
    let mut cycle1 = MyCycle::new(0..2, 1);
    let mut cycle2 = MyCycle::new(6..9,1);
    let mut test_chain = cycle1.chain(cycle2);
    assert_eq!(Some(0),test_chain.next());
    assert_eq!(Some(1),test_chain.next());
    assert_eq!(Some(0),test_chain.next());
    assert_eq!(Some(1),test_chain.next());
    assert_eq!(Some(6),test_chain.next());


    //let test_result = cycle1.my_chain(&mut cycle2);
    //println!("{:?}",test_result);


    /// test zip function

    let mut zipcycle1 = MyCycle::new(0..2, 2);
    let mut zipcycle2 = MyCycle::new(2..4,2);
    let mut zip_result = zipcycle1.zip(zipcycle2);
    assert_eq!(Some((0,2)),zip_result.next());
    assert_eq!(Some((1,3)),zip_result.next());
    assert_eq!(Some((0,2)),zip_result.next());



    // the point here is that, I is an iterator - or whatever it is.
    // we cannot rebuild it as it's given - but we can build a vector from its items
    // this means that we have to implement the trait clone and whatever is needed for I::Item


}
