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

    /// CREATE DIRECTORY
    fn mk_dir_r(path: &str,current_loc:&mut Dir){
        println!("{:?}",path);
        /*
        if path.len() == 0{
            return true;
        }
         */


        let split_path: Vec<&str> = path.split("/").collect();
        if split_path.len() == 1{
            //println!("Split path length is 1 -- creating new directory");
            /// FIRST CHECK CURRENT LOCATION IF ANY OF THE CHILDREN HAVE THE SAME NAME AS CURRENT NEW NAME

           for i in 0..(*current_loc).children.len(){
               match &mut (*current_loc).children[i]{
                   Node::Dir(e) => {
                       /// IF THERE IS ANY, PRINT "DIRECTORY ALREADY EXISTS FOR GIVEN NAME" AND RETURN
                       if e.name == split_path[0]{
                           println!("Error > Directory \"{}\" already exists.",split_path[0]);
                           return;
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
            return;
        }

        /// CHECK BETWEEN THE CURRENT DIRECTORY CHILDREN IF THERE IS ANY DIRECTORY THAT MATCHES CURRENT PATH
        for i in 0..(*current_loc).children.len(){
            /// only check directories
            match &mut (*current_loc).children[i]{
                Node::Dir(e) => {
                    if e.name == split_path[0]{
                        Self::mk_dir_r(split_path[1..].join("/").as_str(),e);
                    }
                }
                _ => {}
            }
        }

        //println!("Returning false at end of recursive call");
        return;
    }
    pub fn mk_dir(&mut self,path: &str){
        // create new directory in file system

        let new_path: Vec<&str> = path.split("/").collect();


        // check if trying to create folder and not a file
        if new_path[new_path.len()-1].contains("."){
            //println!("Invalid path");
            return;
        }
        /// check if root is correct
        if new_path[0] != self.root.name{
            //println!("Invalid file system name");
            return;
        }

        Self::mk_dir_r(new_path[1..].join("/").as_str(),&mut self.root);
        //println!("Returning false as nothing was matched in recursive call");
        return;

    }

    /// REMOVE EMPTY DIRECTORY
    fn rm_dir_r(path: &str, current_loc: &mut Dir){
        println!("{:?}",path);

        let split_path: Vec<&str> = path.split("/").collect();
        if split_path.len() == 0{
            println!("No folder was found to be deleted.");
            return;
        }

        /// CHECK BETWEEN THE CURRENT DIRECTORY CHILDREN IF THERE IS ANY DIRECTORY THAT MATCHES CURRENT PATH
        for i in 0..(*current_loc).children.len(){
            /// only check directories
            match &mut (*current_loc).children[i]{
                Node::Dir(e) => {
                    if e.name == split_path[0] && split_path.len() == 1{
                        if e.children.len() == 0{
                            (*current_loc).children.remove(i);
                            println!("Removed directory succesfully");
                            return;
                        }
                        else{
                            println!("Cannot remove - directory is not empty.");
                            return;
                        }

                    }
                    else if e.name == split_path[0]{
                        Self::rm_dir_r(split_path[1..].join("/").as_str(),e);
                    }
                }
                _ => {}
            }
        }

        //println!("Returning false at end of recursive call");
        return;
    }
    pub fn rm_dir(&mut self,path: &str){
        // remove directory from file system
        let new_path: Vec<&str> = path.split("/").collect();


        // check if trying to create folder and not a file
        if new_path[new_path.len()-1].contains("."){
            //println!("Invalid path");
            return;
        }

        /// check if root is correct
        if new_path.len() == 1 && new_path[0] == self.root.name{
            println!("Cannot remove filesystem folder.");
            return;
        }
        if new_path[0] != self.root.name{
            return;
        }

        Self::rm_dir_r(new_path[1..].join("/").as_str(),&mut self.root);
        //println!("Returning false as nothing was matched in recursive call");
        return;
    }

    /// INSERT FILE IN DIRECTORY
    fn new_file_r(path: &str, current_loc: &mut Dir, file: &File){

        println!("Current location >> {}",(*current_loc).name);

        // if the path contains "/" it means that we still need to look for the right location
        if path.contains("/"){
            let split_path: Vec<&str> = path.split("/").collect();
            if split_path.len() == 1{
                return;
            }
            for i in 0..(*current_loc).children.len(){
                /// only check directories
                match &mut (*current_loc).children[i]{
                    Node::Dir(e) => {
                        if e.name == split_path[0]{
                            Self::new_file_r(split_path[1..].join("/").as_str(),e,file);
                        }
                    }
                    _ => {}
                }
            }
        }
        else{ // if it doesn't contain any "/", it means that we are in the final directory > add the file to the current directory's children.
            (*current_loc).children.push(Node::File(
                File{
                    name: (*file).name.clone(),
                    content: (*file).content.clone(),
                    creation_time: (*file).creation_time.clone(),
                    type_: (*file).type_.clone(),
                }
            ));
            println!("File inserted succesfully.");
        }
        return;

    }
    pub fn new_file(&mut self,path: &str, file: File){
        // create new file inside of file system
        let new_path: Vec<&str> = path.split("/").collect();
        println!("{:?}",new_path);

        if new_path[0] != self.root.name || new_path.len() <= 1{
            return;
        }

        Self::new_file_r(new_path[1..].join("/").as_str(),&mut self.root,&file);
        return;
    }
    fn rm_file_r(path: &str, current_loc: &mut Dir){
        println!("{:?}",path);

        let split_path: Vec<&str> = path.split("/").collect();

        /// CHECK BETWEEN THE CURRENT DIRECTORY CHILDREN IF THERE IS ANY DIRECTORY THAT MATCHES CURRENT PATH
        for i in 0..(*current_loc).children.len(){
            /// only check directories
            match &mut (*current_loc).children[i]{
                /// check between current child directories
                Node::Dir(e) => {
                    if e.name == split_path[0] && split_path.len() == 2{
                        /// if you only have the file remaining in the path, check between the current directory's children and remove that file
                        for x in 0..e.children.len(){
                            match &e.children[x]{
                                Node::File(e_1) => {
                                    if (*e_1).name == split_path[1]{
                                        e.children.remove(x);
                                        println!("File removed succesfully");
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    else if e.name == split_path[0]{
                        Self::rm_file_r(split_path[1..].join("/").as_str(),e);
                    }
                }
                _ => {}
            }
        }

        //println!("Returning false at end of recursive call");
        return;
    }
    pub fn rm_file(&mut self,path: &str){
        // remove file from file system
        let new_path: Vec<&str> = path.split("/").collect();
        println!("{:?}",new_path);

        if new_path[0] != self.root.name || new_path.len() <= 1{
            println!("Invalid path");
            return;
        }

        Self::rm_file_r(new_path[1..].join("/").as_str(),&mut self.root);
        return;
    }

    fn get_file_r<'a>(path: &str, current_loc : &mut Dir, final_reference: &mut Option<&'a mut File>){
        let split_path: Vec<&str> = path.split("/").collect();

        /// CHECK BETWEEN THE CURRENT DIRECTORY CHILDREN IF THERE IS ANY DIRECTORY THAT MATCHES CURRENT PATH
        for i in 0..(*current_loc).children.len(){
            /// only check directories
            match &mut (*current_loc).children[i]{
                /// check between current child directories
                Node::Dir(e) => {
                    if e.name == split_path[0] && split_path.len() == 2{
                        /// if you only have the file remaining in the path, check between the current directory's children and remove that file
                        for x in 0..e.children.len(){
                            match &mut e.children[x]{
                                Node::File(e_1) => {
                                    if (*e_1).name == split_path[1]{
                                        // same as for removing a file, but save the current content of the reference with the address of the found file
                                        (*final_reference) = Some(e_1);
                                        return;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    else if e.name == split_path[0]{
                        Self::get_file_r(split_path[1..].join("/").as_str(),e,final_reference);
                    }
                }
                _ => {}
            }
        }

        //println!("Returning false at end of recursive call");
        return;
    }
    pub fn get_file(&mut self,path: &str,) -> Option<&mut File>{
        // retrieve file from file system
        // same implementation as remove file, but return a
        let new_path: Vec<&str> = path.split("/").collect();
        println!("{:?}",new_path);
        let mut reference_to_file: Option<&mut File> = None;


        if new_path[0] != self.root.name || new_path.len() <= 1{
            println!("Invalid path");
            return reference_to_file;
        }



        Self::get_file_r(new_path[1..].join("/").as_str(),&mut self.root,&mut reference_to_file);
        return reference_to_file

    }

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
    my_fs.mk_dir("Daniele/dir1");
    my_fs.mk_dir("Daniele/dir1/dir2");
    my_fs.mk_dir("Daniele/ciao");

    println!("{:?} \n>>",my_fs.root);

    my_fs.rm_dir("Daniele/dir1/dir2");

    println!("{:?}",my_fs.root);

    // Add file test
    my_fs.new_file("Daniele/ciao/",File{
        name: String::from("ciao.txt"),
        content: vec![],
        creation_time: TIMESTAMP,
        type_: FileType::Text,
    });
    println!("{:?}",my_fs.root);

   /*
   // Remove file test
    my_fs.rm_file("Daniele/ciao/ciao.txt");
    println!("{:?}",my_fs.root);
    */


}
