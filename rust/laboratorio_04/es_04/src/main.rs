extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
use std::thread;
use std::thread::current;

fn compute_single_letters(lines: &Vec<String>, n_threads: usize){

    /// first, calculate how many lines each thread will have
    let mut n_lines_for_thread;
    if lines.len() % n_threads == 0{
        n_lines_for_thread = lines.len() / n_threads;
    }
    else{
        n_lines_for_thread = lines.len() / n_threads + 1;
    }

    /// now, create a vector that will contain the threads
    let mut threads = Vec::new();

    let mut parts: Vec<Vec<String>> = lines.chunks(n_lines_for_thread).map(|x| {
        x.iter().map(|y| {
            y.to_string()
        }).collect()
    }).collect();

    // create threads and push them inside of final result
    for i in 0..n_threads{
        let mut new_hash_map:HashMap<char,i32> = HashMap::new();
        let current_lines = parts.get(i).unwrap().to_vec();
        let new_thread = thread::spawn(move ||single_letter(new_hash_map,current_lines));
        threads.push(new_thread);
    }


    let mut final_hashmap: HashMap<char,i32> = HashMap::new();
    for c_thread in threads{
        let temp_result = c_thread.join().unwrap();
        temp_result.iter().for_each(|e2| *final_hashmap.entry(*e2.0).or_insert(*e2.1) += e2.1 )
    }



    println!("Printing total letters found in map");
    final_hashmap.iter().for_each(|e| println!("{} - {}",e.0,e.1));
}


fn single_letter(mut current_counter: HashMap<char,i32>, lines: Vec<String>) ->HashMap<char,i32>{
    /// calculate for the given lines the total amount of letters.
    lines.iter().for_each(|x| {
        // for each line inside of lines, read every word increase the current counter by 1.
        x.clone().chars().for_each(|y| *current_counter.entry(y).or_insert(1) += 1)
    });
    return current_counter;
}
fn main() {
    /*
    l'idea è quella di avere un file iniziale di partenza composto da una quantità conosciuta di linee
    dividiamo quindi quelle linee per il numero di thread che vogliamo provino ad eseguire in contemporanea il calcolo delle lettere

    una volta creati i diversi chunk, creiamo un vettore che andrà a contenere i nostri thread
    creiamo quindi la quantità predefinita di thread, e li assegniamo in un vettore, passandoci come funzione da eseguire
    il calcolo delle lettere totali sulla porzione a lui assegnata

    questi thread, una volta finiti, restituiranno come valore di ritorno una HashMap contenente tutte le frequenze delle lettere
    presenti nelle linee analizzate
     */
    println!("Hello, world!");
    /*
    ...
    ...
    ...
    ...
     */
    let my_file = File::open("inputfile.txt");
    if my_file.is_ok(){
        let mut file = my_file.unwrap();
        let lines = BufReader::new(&file).lines().count(); /// read lines then go back to beginning
        file.rewind().unwrap();
        let mut lines_strings: Vec<String> = Vec::new();
        for line in BufReader::new(&file).lines(){ /// add all lines to Vec<string> to pass to function
            if line.is_ok(){
                lines_strings.push(line.unwrap());
            }
        }
        //lines_strings.iter().for_each(|e| println!("{:?}",e));

        // i now have a list of all lines - i now need a function that takes in this list + a number of lines
        compute_single_letters(&lines_strings,3);
    }
    else{
        println!("Error opening file");
        return;
    }

}
