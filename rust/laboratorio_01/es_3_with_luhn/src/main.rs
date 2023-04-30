use std::env::args;
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let _t1: char = String::from(minefield[0]).pop().unwrap(); // get rows
    let nr_rows: u32 = _t1.to_digit(32).unwrap();
    let _t2: char = String::from(minefield[1]).pop().unwrap(); // get cols
    let nr_cols: u32 = _t2.to_digit(32).unwrap();

    let char_list = minefield[2];
    let mut str_matrix: Vec<&str> = Vec::new();
    let mut total_items:usize =( nr_rows*nr_cols) as usize;
    let mut current_items:usize = 0;
    loop{
        if current_items >= total_items{break;}
        str_matrix.push(&char_list[current_items..(current_items+(nr_cols as usize))]);
        current_items = current_items + (nr_cols as usize);
    }
    let mut current_row:usize = 0;
    let mut current_col:usize = 0;
    let mut count: u32 = 0;
    let mut chars_list: Vec<char>;
    let mut final_result: Vec<String> = Vec::new();
    loop{
        if current_row >= (nr_rows as usize) {break;}
        current_col = 0;
        chars_list = str_matrix[current_row].chars().collect();
        loop{
            if current_col >= (nr_cols as usize){break;}
            if chars_list[current_col] == '-'{
                if current_row == 0{ /// STARTING ROW
                    // code to check around all of the current cell
                    if (current_col + 1) <= (nr_cols - 1) as usize{
                        if chars_list[current_col + 1] == '*'{count = count + 1;}
                    }
                    if current_col != 0{
                        if chars_list[current_col - 1] == '*'{count = count + 1;}
                    }

                    /*
                    ABOVE ROW IS NOT POSSIBLE
                    let mut chars_aboverow: Vec<char> = str_matrix[current_row - 1].chars().collect();
                    if chars_aboverow[current_col] == '*'{count = count + 1;}
                    if (current_col + 1) <= (nr_cols - 1) as usize{ if chars_aboverow[current_col - 1] == '*' { count = count + 1; } }
                    if current_col != 0{ if chars_aboverow[current_col + 1] == '*' { count = count + 1; } }
                     */


                    // check below row
                    let mut chars_belowrow: Vec<char> = str_matrix[current_row + 1].chars().collect();
                    if chars_belowrow[current_col] == '*'{count = count + 1;}
                    if current_col != 0{ if chars_belowrow[current_col - 1] == '*' { count = count + 1; } }
                    if (current_col + 1) <= (nr_cols - 1) as usize{ if chars_belowrow[current_col + 1] == '*' { count = count + 1; } }

                }
                else{/// MIDDLE ROWS
                    if current_row > 0 && current_row < ((nr_rows - 1 ) as usize){
                        // code to check around all of the current cell
                        if (current_col + 1) <= (nr_cols - 1) as usize{
                            if chars_list[current_col + 1] == '*'{count = count + 1;}
                        }
                        if current_col != 0{
                            if chars_list[current_col - 1] == '*'{count = count + 1;}
                        }

                        // check other rows
                        let mut chars_aboverow: Vec<char> = str_matrix[current_row - 1].chars().collect();
                        if chars_aboverow[current_col] == '*'{count = count + 1;}
                        if (current_col + 1) <= (nr_cols - 1) as usize{
                            if chars_aboverow[current_col + 1] == '*' { count = count + 1; } }
                        if current_col != 0{
                            if chars_aboverow[current_col - 1] == '*' { count = count + 1; } }
                        // check below row
                        let mut chars_belowrow: Vec<char> = str_matrix[current_row + 1].chars().collect();
                        if chars_belowrow[current_col] == '*'{count = count + 1;}
                        if current_col != 0{
                            if chars_belowrow[current_col - 1] == '*' { count = count + 1; } }
                        if (current_col + 1) <= (nr_cols - 1) as usize{
                            if chars_belowrow[current_col + 1] == '*' { count = count + 1; } }
                    }
                    else if current_row == ((nr_rows - 1) as usize){/// FINAL ROWS
                        // code to check around all of the current cell
                        if (current_col + 1) <= (nr_cols - 1) as usize{
                            if chars_list[current_col + 1] == '*'{count = count + 1;}
                        }
                        if current_col != 0{
                            if chars_list[current_col - 1] == '*'{count = count + 1;}
                        }

                        // check other rows
                        let mut chars_aboverow: Vec<char> = str_matrix[current_row - 1].chars().collect();
                        if chars_aboverow[current_col] == '*'{count = count + 1;}
                        if (current_col + 1) <= (nr_cols - 1) as usize{
                            if chars_aboverow[current_col + 1] == '*' { count = count + 1; }
                        }
                        if current_col != 0{
                            if chars_aboverow[current_col - 1] == '*' { count = count + 1; }
                        }

                        /*
                        BELOW ROW NOT POSSIBLE
                        let mut chars_belowrow: Vec<char> = str_matrix[current_row + 1].chars().collect();
                        if chars_belowrow[current_col] == '*'{count = count + 1;}
                        if current_col != 0{ if chars_belowrow[current_col - 1] == '*' { count = count + 1; } }
                        if (current_col + 1) <= (nr_cols - 1) as usize{ if chars_belowrow[current_col + 1] == '*' { count = count + 1; } }
                         */

                    }
                }
                if count > 0{
                    chars_list[current_col] = char::from_digit(count,32).unwrap();
                }
                count = 0;
            }
            //print!("{}",chars_list[current_col]);
            current_col = current_col +1;
        }
        /// create new String
        let mut new_string: String = String::new();
        /// transform Vec<char> into String
        for x in chars_list{new_string.push(x);}
        /// save current String in final result.
        final_result.push(new_string);
        current_row = current_row + 1;
    }
    final_result
}

fn main() {

    let args: Vec<String> = args().skip(1).collect();
    let args_as_str: Vec<&str> = args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    // >> questo metodo però va a creare una copia di ogni stringa

    /// OPPURE POSSO CREARE DIRETTAMENTE UN VETTORE DI &STR
    /// E ITERARE SU args MA USANDO UN CICLO FOR() con un iteratore che va da 0 al numero di argomenti >>
    /// PER OGNI ELEMENTO, PRENDO UNA SLICE CON &args[...] con dentro come indici l'iteratore*n.cols ... (iteratore+1)*n.cols
    /// facendo così, essendo una slice, il risultato sarà una reference ad un pezzo di memoria -> essendo che mi riferisco però
    /// a args, che è al di fuori del for(), i valori inseriti dentro il nuovo array avranno una lifetime sufficiente allo sviluppo del programma.


    // ["--rows=3", "--cols=3", "   * "]
    if args.len() == 3{
        let final_result: Vec<String> = annotate(&args_as_str);
        for x in final_result{println!("{}",x);}
    }
    else { println!("Not enough args!"); }
}
