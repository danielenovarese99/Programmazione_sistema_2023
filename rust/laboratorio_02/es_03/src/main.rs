use std::ops::Deref;
use std::thread::current;
use std::time::Instant;

static TIMESTAMP:u64 = 0;

enum FileType {
    Text, Binary
}
#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
enum Node {
    File(File),
    Dir(Dir),
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
    pub fn mk_dir(&mut self,path: &str) -> Result<T, E>{
        // create new directory in file system

        let new_path: Vec<String> = path.clone().split("/").collect();
        if new_path[new_path.len()-1].contains("."){
            return Err("Cannot create a new directory - invalid name");
        }

        if mk_dir_r(path,&mut self.root) == true{
            return Ok(());
        }

        Err("invalid path")

    }
    fn mk_dir_r(path: &str,current_loc:&mut Dir) -> bool{
        if path.is_empty(){
            return true;
        }


        let split_path: Vec<String> = path.clone().split("/").collect();
        if split_path.len() == 0{
            return false;
        }
        else if split_path.len() == 1{
            if split_path[0] == (*current_loc).deref().name{
                (*current_loc).children.push(
                    Node::Dir(Dir::new(&split_path[0].as_str(),TIMESTAMP,vec![]))
                )
            }
        }

        // if the current directory is the first part of the path, proceed downwards
        if (*current_loc).name == split_path[0] {
            for i in 0..(*current_loc).deref().children.len(){
                /// only check directories
                if Node::Dir == (*current_loc).children[i] {
                    if (*current_loc).children[i].name == split_path[1]{
                        return mk_dir_r(path: &split_path[1..],(*current_loc).children[i]); // Todo check here
                    }
                }
            }
        }



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
    println!("Hello, world!");
}
