use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader};
/**
A calendar is basically made from tuples of integers, representing HH:MM format
**/
struct Calendar{
    schedule: Vec<(i32,i32)>,
    bounds: (i32,i32)
}


fn is_overlapped(t1 : (i32,i32), t2: (i32,i32))->bool{
    /// (8,30) - (9,0)
    if t1.0 < t2.0 {return false;}
    else{
        if t1.0 == t2.0{
            if t1.1 < t2.1{return false;}
            else{
                if t1.1 == t2.1{return false;}
                else{return true;}
            }
        }
        else{return true;}
    }
}
impl Calendar{
    /*

     /**
    **From file method
    **First two lines are start and end of schedule
    *** all other lines >> line 0 start of appointment
                        >> line 1 end of appointment
    so read 2 lines, create item and insert into Vec<(i32,i32)>
    **/

     */

     fn from_file(filename : String)->Self{
        let mut calendar = File::open(filename).expect("Could not open file");
        let calendar_reader = BufReader::new(calendar);
        let mut lines = calendar_reader.lines();

        let unformatted_bound1 = lines.next().unwrap().unwrap();
        let unformatted_bound2 = lines.next().unwrap().unwrap();

        let mut temp_bound1 = unformatted_bound1.split(":");
        let tb1_i: i32 = temp_bound1.next().unwrap().parse().unwrap();

        let mut temp_bound2 = unformatted_bound2.split(":");
        let tb2_i: i32 = temp_bound2.next().unwrap().parse().unwrap();



        let mut appointments: Vec<String> = Vec::new();
        let mut final_appointments: Vec<(i32,i32)> = Vec::new();

        let mut temp = lines.next();
        while temp.is_some(){
            //println!("{:?}",temp.unwrap().unwrap());
            let val = temp.unwrap().unwrap().clone();
            if val.len() > 0{
                appointments.push(val);
            }
            temp = lines.next();
        }

        appointments.into_iter().for_each(|e| {
           let mut temp = e.split(":");
            let val1 = temp.next().unwrap();
            let val2 = temp.next().unwrap();

            let i1: i32 = val1.parse().unwrap();
            let i2: i32 = val2.parse().unwrap();

            final_appointments.push((i1,i2));
        });


        return Calendar{
            schedule: final_appointments,
            bounds: (tb1_i,tb2_i),
        };
    }

     fn check_daily_availability(self,duration_str: String)->Vec<((i32,i32),(i32,i32))>{
        let duration: i32 = duration_str.parse().unwrap();
        let mut result: Vec<((i32,i32),(i32,i32))> = Vec::new();

        let mut i = 0;
        loop{
            if i == self.schedule.len()-1{// last case - check final bound
                let mut t = self.schedule[i];
                if t.1 + duration > 60{
                    t.0 += 1;
                    t.1 = t.1 + duration - 60;
                }
                if is_overlapped(t,(self.bounds.1,0)) == false{
                    result.push(((t.0,0),(self.bounds.1,0)));
                }
                break;
            }
             if i == 0{ // starting case - check initial bound
                let t = (self.bounds.0,0+duration);
                if is_overlapped(t,self.schedule[i]) == false{
                    result.push(((self.bounds.0,0),self.schedule[i]));
                }
            }

            if i % 2 == 1 && i != self.schedule.len() - 1{ // default case - check with end point of current appointment and next one
                let mut t = self.schedule[i];
                if t.1 + duration > 60{
                    t.0 += 1;
                    t.1 = t.1 + duration - 60;
                }
                if is_overlapped(t,self.schedule[i+1]) == false{
                    result.push(((t.0,self.schedule[i].1),self.schedule[i+1]))
                }
            }
            i += 1;
        }

        result
    }
     fn check_mutual_availability(self,duration_str:String,calendar2: Calendar)->Vec<((i32,i32),(i32,i32))>{
        /// obtain possible appointment time intervals from schedule with less time in it - checking bounds
        /// then compare those times wit the ones obtained by comparing with the other calendar
        /// return the result
        let mut result: Vec<((i32,i32),(i32,i32))>;
        if self.bounds.0 > calendar2.bounds.0{
            result = self.check_daily_availability(duration_str.clone());
        }else if self.bounds.0 == calendar2.bounds.0{
            if self.bounds.1 > calendar2.bounds.1{
                result = self.check_daily_availability(duration_str.clone());
            }else{
                result = calendar2.check_daily_availability(duration_str.clone());
            }
        }else{
            result = calendar2.check_daily_availability(duration_str.clone());
        }
        result
    }
}
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("args > {:?}",args);
    if args.len() == 3{
        let c1 = Calendar::from_file(args[0].clone());
        let c2 = Calendar::from_file(args[1].clone());
        let duration = args[2].clone();

        let result: Vec<((i32,i32),(i32,i32))> = c1.check_mutual_availability(duration,c2);
        println!("{:?}",result);
    }
    else{
        println!("Please insert two calendar names and a total appointment duration (MINUTES)");
    }

}
