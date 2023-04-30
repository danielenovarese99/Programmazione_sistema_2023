use std::env::args;
use std::fs::File;
use std::io::{Read,Result};
use std::mem;
use std::fmt::{Debug, Formatter};
// we use read to read bites from file - Result to have
// a specific type for our I/O operations with the CData structure -
// so we don't have to handle the Error case of the <Result> datatype.

#[derive(Copy,Debug,Clone)]
struct ValueStruct{
    t: i32,
    value: f32,
    timestamp: i32,
}
#[derive(Copy,Debug,Clone)]
struct MValueStruct{
    t: i32,
    value: [f32;10],
    timestamp: i32,
}
#[derive(Copy,Debug,Clone)]
struct MessageStruct{
    t: i32,
    message: [u8;21],
}

union ImportData{
    single_val: ValueStruct,
    multiple_val: MValueStruct,
    message: MessageStruct
}

struct CData{
    t: i32,
    data: ImportData,
}
impl Debug for CData{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.t{
            1 => {
                unsafe { println!("Data type : {}; Value : {}, Timestamp : {}",self.t,self.data.single_val.value,self.data.single_val.timestamp);}
            },
            2 => {
                unsafe {
                    println!("Data type : {}; Values : ",self.t);
                    for i in 0..10{
                        print!("{}",self.data.multiple_val.value[i]);
                    }
                    print!("; Timestamp : {}",self.data.multiple_val.timestamp);
                }
            },
            3 => {
                unsafe{
                    println!("Data type : {}; ",self.t);
                    for i in 0..21{
                        print!("{}",self.data.message.message[i] as char);
                    }
                }
            },
            _ => {println!("No option available - invalid type ( {} ) ",self.t);}
        }
        Ok(())
    }
}
impl CData{
    fn from_file(mut file: File) -> Result<CData>{
        // create new structure that will be returned
        let mut temp_cdata: CData = CData{
            t: 0,
            data: ImportData{
                single_val: ValueStruct{
                    t: 0,
                    value: 0.0,
                    timestamp: 0,
                }
            }
        };
        // create buffer of 4 bites to read type (t)
        let mut type_buf = [0u8;4];
        file.read_exact(&mut type_buf)?;
        // allocate that type to the temp struct
        temp_cdata.t = i32::from_le_bytes(type_buf[0..4].try_into().unwrap());
        // switch on cdata.t >>>

        match temp_cdata.t{
            // (1),(2),(3) allocate different buffers depending on size
            1 => {
                let mut new_buffer = [0u8; mem::size_of::<ValueStruct>()- 4];
                file.read_exact(&mut new_buffer);

                let mut temp_single_val = ValueStruct{
                    t: temp_cdata.t.clone(),
                    value: f32::from_le_bytes(new_buffer[0..4].try_into().unwrap()),
                    timestamp: i32::from_le_bytes(new_buffer[4..8].try_into().unwrap()),
                };
                temp_cdata.data = ImportData{single_val: temp_single_val};
                unsafe { println!("{:?}",temp_cdata.data.single_val);}
            },
            2 => {
                let mut new_buffer = [0u8; mem::size_of::<MValueStruct>() - 4];
                file.read_exact(&mut new_buffer)?;

                let mut multiple_vals = [0.0f32;10];
                for i in 0..10{
                    multiple_vals[i] = f32::from_le_bytes(new_buffer[0+(4*i)..4+(4*i)].try_into().unwrap());
                }

                let mut temp_multiple_val = MValueStruct{
                    t: temp_cdata.t.clone(),
                    value: multiple_vals,
                    timestamp: i32::from_le_bytes(new_buffer[40..44].try_into().unwrap()),
                };

                temp_cdata.data = ImportData{multiple_val: temp_multiple_val};
                unsafe {println!("{:?}",temp_cdata.data.multiple_val);}
            }
            3 => {
                let mut new_buffer = [0u8; mem::size_of::<MessageStruct>() - 4];
                file.read_exact(&mut new_buffer)?;

                let mut temp_message = [0u8;21];
                for i in 0..21{
                    temp_message[i] = u8::from_le_bytes([new_buffer[0+i]]);
                }

                let temp_message_struct = MessageStruct{
                    t: temp_cdata.t.clone(),
                    message: temp_message
                };
                temp_cdata.data = ImportData{message: temp_message_struct};
                unsafe {println!("{:?}",temp_cdata.data.message);}
            }
           _ => {println!("Unrecognized type - {}",temp_cdata.t);}
        }
        Ok(temp_cdata)
    }
}

fn main() {
    let mut args: Vec<String> = args().collect();
    if args.len() > 1{
        println!("Args inserted - trying to read file...\n\n");
        let myfile = File::open(args[1].clone()).unwrap();

        let mut my_data = Vec::with_capacity(100);
        for i in 0..100{
            let temp_data: CData = CData::from_file(myfile.try_clone().unwrap()).unwrap();
            // we need to clone the file otherwise we will lose it
            // after using the function due to the borrow checker.
            // (we can't return it as we are returning a value of type struct CData)
            my_data.push(temp_data);
        }
        println!("Process complete.");
    }
    else{
        println!("No args inserted");
    }

}
