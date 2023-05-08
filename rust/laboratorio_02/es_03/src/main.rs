use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::thread::current;
use std::time::Instant;


static TIMESTAMP:u64 = 0;
#[derive(Clone)]
enum FileType {
    Text, Binary
}
#[derive(Clone)]
struct File {
    name: String,
    content: Vec<u8>, // max 1000 bytes, rest of the file truncated
    creation_time: u64,
    type_: FileType,
}
impl File{
    fn new(name: &str, content: Vec<u8>, creation_time: u64, type_: FileType) -> File{
        File{
            name: String::from(name),
            content,
            creation_time,
            type_,
        }
    }
}
impl Debug for File{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("FILE : Name : {}, Creation_time: {}",self.name,self.creation_time);
        Ok(())
    }
}

#[derive(Clone)]
struct Dir {
    name: String,
    creation_time: u64,
    children:  Vec<Node>,
}
impl Dir{
    fn new(name: &str, creation_time: u64, children: Vec<Node>) -> Dir{
        Dir{
            name: String::from(name),
            creation_time,
            children,
        }
    }
}
impl Debug for Dir{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        print!("\nDIR : Name : {}, Creation time : {}, Children : [ ",self.name,self.creation_time);
        for i in 0..self.children.len(){
            println!("{:?} ",self.children[i]);
        }
        print!("]");

        Ok(())
    }
}
#[derive(Clone)]
enum Node {
    File(File),
    Dir(Dir),
}
impl Debug for Node{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Node::Dir(e) => {
                println!("{:?}",e)
            },
            Node::File(e) => {
                println!("{:?}",e)
            }
        }

        Ok(())
    }
}
struct FileSystem {
    root: Dir
}

struct MatchResult<'a>{
    /// TODO >> remember that the strings, nodes  and matchresult created from the query itself
    /// will need to have the same lifetime
    queries: Vec<&'a str>,
    nodes: Vec<&'a mut Node>,

}


impl FileSystem{

    // TODO > remember > handle path strings by passing through nodes each "/", checking if the current node is a directory and its name
    // if it exists ecc.
    // TODO >> do this in functions that require a path
    pub fn new(fs_name: &str) -> FileSystem{
        // create new EMPTY file system
        let mut new_file_system = FileSystem{
            root: Dir::new(fs_name,0,vec![])
        };
        new_file_system
    }
    /*
    pub fn from_dir(path: &str) -> FileSystem{
        // create file system from existing directory
        /// TODO >> this needs to copy all files inside of folder, // RECURSIVELY //
    }
     */

    fn mk_dir_r(path: &str,current_loc:&mut Dir) -> bool{
        println!("{:?}",path);
        /*
        if path.len() == 0{
            return true;
        }
         */


        let split_path: Vec<&str> = path.split("/").collect();
        if split_path.len() == 1{
            println!("Split path length is 1 -- creating new directory");
            /// FIRST CHECK CURRENT LOCATION IF ANY OF THE CHILDREN HAVE THE SAME NAME AS CURRENT NEW NAME

           for i in 0..(*current_loc).children.len(){
               match &mut (*current_loc).children[i]{
                   Node::Dir(e) => {
                       /// IF THERE IS ANY, PRINT "DIRECTORY ALREADY EXISTS FOR GIVEN NAME" AND RETURN
                       if e.name == split_path[0]{
                           println!("Error > Directory \"{}\" already exists.",split_path[0]);
                           return false;
                       }
                   }
                   _ => {}
               }
           }
            /// IF NOT, CREATE NEW DIRECTORY
            let mut new_node: Node = Node::Dir(
                Dir{
                    name: String::from(split_path[0].clone()),
                    children: vec![],
                    creation_time: TIMESTAMP,
                }
            );
            (*current_loc).children.push(new_node);
            return true;
        }

        /// CHECK BETWEEN THE CURRENT DIRECTORY CHILDREN IF THERE IS ANY DIRECTORY THAT MATCHES CURRENT PATH
        for i in 0..(*current_loc).children.len(){
            /// only check directories
            match &mut (*current_loc).children[i]{
                Node::Dir(e) => {
                    if e.name == split_path[0]{
                        let result = Self::mk_dir_r(split_path[1..].join("/").as_str(),e);
                        return result;
                    }
                }
                _ => {}
            }
        }

        println!("Returning false at end of recursive call");
        false
    }

    pub fn mk_dir(&mut self,path: &str) -> bool {
        // create new directory in file system

        let new_path: Vec<&str> = path.split("/").collect();


        // check if trying to create folder and not a file
        if new_path[new_path.len()-1].contains("."){
            println!("Invalid path");
            return false
        }
        /// check if root is correct
        if new_path[0] != self.root.name{
            println!("Invalid file system name");
            return false;
        }

        let result: bool = Self::mk_dir_r(new_path[1..].join("/").as_str(),&mut self.root);
        if result == true{
            return true;
        }

        println!("Returning false as nothing was matched in recursive call");
        false

    }


    /*
    fn rm_dir(&mut self,path: &str) -> Result<()>{
        // remove directory from file system
    }

     */
    /*
    fn new_file(&mut self,path: &str, file: File) -> Result<()>{
        // create new file inside of file system
    }
     */
    /*
    fn rm_file(&mut self,path: &str) -> Result<()>{
        // remove file from file system
    }
     */
    /*
    fn get_file(&mut self,path: &str) -> Option<&mut File>{
        // retrieve file from file system
    }
     */
    /*
    fn search(&mut self,queries: &[&str]) -> Option<MatchResult>{
        // cerca dei file che matchano le query indicate e restituisce un oggetto MATCHRESULT
        // con un riferimento mutabile ai file trovati

        /*
        TYPES OF QUERIES
        "name:string" >> find all files that have that string inside of name
        "content: string" >> find all files that contain that string inside of them
        "larger: val" >> find all files with size larger than val
        "smaller: val" >> find all files with size smaller than val
        "newer: val" >> find files created before val
        "older: val" >> find files created after val
         */
    }
     */
}
fn main() {
    let mut my_fs: FileSystem = FileSystem::new("Daniele");
    let my_result: bool = my_fs.mk_dir("Daniele/dir1");
    let my_second_result : bool = my_fs.mk_dir("Daniele/dir1/dir1");
    let test_this = my_fs.mk_dir("Daniele/ciao");

    println!("{:?}",my_fs.root);
}
