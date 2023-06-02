use std::thread;

pub fn permutazioni_semplici(pos: i32, val: Vec<i32>, mut sol: Vec<String>, mut mark: Vec<i32>, count : i32){
    if pos >= val.len() as i32 {    /// ending point - all items must be used and final result must be 10
        if count == 10{
            println!("Possibile soluzione : {:?}",sol);
            return;
        }
    }

    for i in 0..val.len(){    /// recursion
        if mark[i] == 0{
            if count + val[i] <= 10{
                mark[i] = 1;
                sol.push("+".parse().unwrap());
                sol.push(val[i].to_string());
                //permutazioni_semplici(pos+1,val.clone(),sol.clone(),mark.clone(),count+val[i]);
                if i % 2 == 0{
                    let mut mark_clone = mark.clone();
                    let mut sol_clone = sol.clone();
                    let val_clone = val.clone();
                    let new_count = count + val[i];
                    let thread_even = thread::spawn(move|| permutazioni_semplici(pos+1,val_clone,sol_clone,mark_clone,new_count));
                    thread_even.join().unwrap();
                }
                else{
                    permutazioni_semplici(pos+1,val.clone(),sol.clone(),mark.clone(),count+val[i]);
                }
                mark[i] = 0;
            }
            else if count - val[i] <= 10{
                mark[i] = 1;
                sol.push("-".parse().unwrap());
                sol.push(val[i].to_string());
                //permutazioni_semplici(pos+1,val.clone(),sol.clone(),mark.clone(),count-val[i]);
                if i % 2 == 0{
                    let mut mark_clone = mark.clone();
                    let mut sol_clone = sol.clone();
                    let val_clone = val.clone();
                    let new_count = count - val[i];
                    let thread_even = thread::spawn(move|| permutazioni_semplici(pos+1,val_clone,sol_clone,mark_clone,new_count));
                    thread_even.join().unwrap();
                }
                else{
                    permutazioni_semplici(pos+1,val.clone(),sol.clone(),mark.clone(),count+val[i]);
                }
                mark[i] = 0;
            }
            else if count * val[i] <= 10{
                mark[i] = 1;
                sol.push("*".parse().unwrap());
                sol.push(val[i].to_string());
                //permutazioni_semplici(pos+1,val.clone(),sol.clone(),mark.clone(),count*val[i]);
                if i % 2 == 0{
                    let mut mark_clone = mark.clone();
                    let mut sol_clone = sol.clone();
                    let val_clone = val.clone();
                    let new_count = count * val[i];
                    let thread_even = thread::spawn(move|| permutazioni_semplici(pos+1,val_clone,sol_clone,mark_clone,new_count));
                    thread_even.join().unwrap();
                }
                else{
                    permutazioni_semplici(pos+1,val.clone(),sol.clone(),mark.clone(),count+val[i]);
                }
                mark[i] = 0;
            }
            else if count / val[i] <= 10{
                mark[i] = 1;
                sol.push("/".parse().unwrap());
                sol.push(val[i].to_string());
                //permutazioni_semplici(pos+1,val.clone(),sol.clone(),mark.clone(),count/val[i]);
                if i % 2 == 0{
                    let mut mark_clone = mark.clone();
                    let mut sol_clone = sol.clone();
                    let val_clone = val.clone();
                    let new_count = count / val[i];
                    let thread_even = thread::spawn(move|| permutazioni_semplici(pos+1,val_clone,sol_clone,mark_clone,new_count));
                    thread_even.join().unwrap();
                }
                else{
                    permutazioni_semplici(pos+1,val.clone(),sol.clone(),mark.clone(),count+val[i]);
                }
                mark[i] = 0;
            }
        }
    }


}
fn main() {
    println!("Hello, world!");
    let test_vec : Vec<i32> = vec![2,7,2,2,1];
    let mut mark:Vec<i32> = Vec::with_capacity(test_vec.len());
    for i in 0..test_vec.len(){mark.push(0)}
    let mut sol: Vec<String> = Vec::new();

    permutazioni_semplici(0,test_vec,sol,mark,0);
    /*
    cifre comprese tra 0-9, ripetute anche
    devono essere utilizzate TUTTE (permutazioni)
    no limiti su operazioni (anche somma va bene)
    vettore soluzione è un vettore di Stringhe che contiene l'operazione totale che ha portato a quel risultato
    non si tiene conto della precedenza degli operatori
     */

}
