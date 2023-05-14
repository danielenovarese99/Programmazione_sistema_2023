extern crate core;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::str::FromStr;
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


    fn from(f: &File) -> Self{
        File{
            name: f.name.clone(),
            creation_time: f.creation_time.clone(),
            content: f.content.clone(),
            type_: f.type_.clone()
        }
    }
}
impl Debug for File{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("FILE : Name : {}, Creation_time: {}",self.name,self.creation_time);
        Ok(())
    }
}
struct Query{
    name: Vec<String>,
    content: Vec<String>,
    larger: Vec<u64>,
    smaller: Vec<u64>,
    newer: Vec<u64>,
    older: Vec<u64>,
}
impl Debug for Query{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        print!("Name queries: [");
        for i in 0..self.name.len(){print!("{} - ",self.name[i])};
        println!(" ]");
        print!("Content queries: [");
        for i in 0..self.content.len(){print!("{} - ",self.content[i])};
        println!(" ]");
        print!("Larger queries: [");
        for i in 0..self.larger.len(){print!("{} - ",self.larger[i])};
        println!(" ]");
        print!("Smaller queries: [");
        for i in 0..self.smaller.len(){print!("{} - ",self.smaller[i])};
        println!(" ]");
        print!("Newer queries: [");
        for i in 0..self.newer.len(){print!("{} - ",self.newer[i])};
        println!(" ]");
        print!("Older queries: [");
        for i in 0..self.older.len(){print!("{} - ",self.older[i])};
        println!(" ]");
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
    fn from(d: &Dir) -> Self{
        Dir{
            name: d.name.clone(),
            creation_time: d.creation_time.clone(),
            children: d.children.clone(),
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
enum Node {
    File(File),
    Dir(Dir),
}

impl Clone for Node {
    fn clone(&self) -> Self {
        match self {
            Node::File(f) => {Node::File(File::from(f))}
            Node::Dir(d) => {Node::Dir(Dir::from(d))}
        }
    }
}
impl Node{
    fn is_dir(&self) -> bool{
        match self {
            Node::File(e) => {false},
            Node::Dir(e) => {true},
        }
    }

    fn name(&self) -> &String{
        match self{
            Node::Dir(E) => {&E.name},
            Node::File(E) => {&E.name},
        }
    }
    fn content(&self) -> Option<&Vec<u8>>{
        match self{
            Node::File(E) => {Some(&E.content)}
            Node::Dir(E) => None
        }
    }
    fn creation_time(&self) -> &u64{
        match self{
            Node::File(E) => {&E.creation_time}
            Node::Dir(E) => {&E.creation_time}
        }
    }

    fn get_mut_file(node: &mut Node) -> Option<&mut File>{
        match node{
            Node::Dir(E) => None,
            Node::File(E) => Some(E),
        }
    }
    fn get_mut_dir(node: &mut Node) -> Option<&mut Dir>{
        match node{
            Node::Dir(E) => Some(E),
            Node::File(E) => None,
        }
    }
    fn get_file(node: &Node) -> Option<&File>{
        match node{
            Node::Dir(E) => None,
            Node::File(E) => Some(&E)
        }
    }

    fn get_dir(node: &Node) -> Option<&Dir>{
        match node{
            Node::File(E) => None,
            Node::Dir(E) => Some(&E)
        }
    }

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

    /// GET FILE
    /*
    What was the problem?
    The problem was that, by using a match() construct, i was assigning a mutable address to current_loc.children[] -> doing so was
    not making me able to return the actual return value of the function, which was a mutable address to a file - which was inevitably
    contained inside of &mut current_loc.children[]

    How did i get around this?
    Making the match() actually a function that returns either a bool or a &mut File / Dir, always releasing the mut
    and not leaving it inside of the current scope.
     */
    fn get_file_r<'a>(path: &str, current_loc : &'a mut Dir) -> Option<&'a mut File>{
        if path.contains("/"){
            let split_path: Vec<&str> = path.split("/").collect();

            /// CHECK BETWEEN THE CURRENT DIRECTORY CHILDREN IF THERE IS ANY DIRECTORY THAT MATCHES CURRENT PATH
            for i in 0..current_loc.children.len(){
                /// only check directories
                if current_loc.children[i].is_dir() && current_loc.children[i].name().eq(split_path[0]){
                    println!("Going down path... => {}",split_path[0]);
                    let new_path = split_path[i..].join("/");
                    return Self::get_file_r(new_path.as_str(),Node::get_mut_dir(&mut current_loc.children[i]).unwrap());
                }
            }
        }
        else{
            for i in 0..current_loc.children.len(){
                if current_loc.children[i].is_dir() == false && Node::get_mut_file(&mut current_loc.children[i]).unwrap().name == path{
                    return Node::get_mut_file(&mut current_loc.children[i])
                }
            }
        }
        // nothing found
        return None
    }
    pub fn get_file(&mut self,path: &str,) -> Option<&mut File>{
        // retrieve file from file system
        // same implementation as remove file, but return a &mut File
        let new_path: Vec<&str> = path.split("/").collect();
        println!("{:?}",new_path);



        if new_path[0] != self.root.name || new_path.len() <= 1{
            println!("Invalid path");
            return None;
        }


        let path_f = new_path[1..].join("/");
        let result = Self::get_file_r(path_f.as_str(),&mut self.root);

        if result.is_some(){
            return result;
        }
        return None

    }

    /*
    fn search_r<'a>(current_loc: &'a mut Dir, final_result: &'a mut MatchResult<'a>, queries : &Query){
        /// TODO > finish implementation
        /// remember - only check for files...
        for child in current_loc.children.iter_mut(){
            if child.is_dir(){ /// DIRECTORY >> SEARCH IN DEPTH INSIDE THAT DIRECTORY
                Self::search_r(Node::get_mut_dir(child).unwrap(),final_result, queries);
            }
            else{
                /// we have a file
                /// iter through entire query - if any of them match, add it to the current final option
                // names
                for i in 0..queries.name.len(){
                    if child.name().as_str() == queries.name[i].as_str(){
                        final_result.nodes.push(child);
                        println!("Found a file matching with a name query");
                        break;
                    }
                }
            }
        }
        /*
        if current_loc.children[i].is_dir() == false && Node::get_mut_file(&mut current_loc.children[i]).unwrap().name == path{
                    return Node::get_mut_file(&mut current_loc.children[i])
                }
         */
    }
     */
    pub fn search<'a>(&mut self,queries: &[&'a str]){
        // cerca dei file che matchano le query indicate e restituisce un oggetto MATCHRESULT
        // con un riferimento mutabile ai file trovati

        /// create function for recursive file search that takes in input a parameter (search parameter, match while looking through items)
        /// return Option <&mut Node>
        /// same layout as get function, but instead of using get_mut_file, just return Some<&mut current_loc.children[i]>

        // i have multiple queries - if one matches a file, that file is retrieved

        // you don't have the problem of trying to retrieve multiple times the same file, as there is only one iteration over the items.

        let mut final_result  = MatchResult{
            queries: vec![],
            nodes: vec![],
        };
        for i in 0..queries.len(){
            final_result.queries.push(queries[i].clone());
        }

        let mut myquery: Query = Query{
            name: vec![],
            content: vec![],
            larger: vec![],
            smaller: vec![],
            newer: vec![],
            older: vec![],
        };

        /// handle all inserted queries, then insert them in a proper struct
        for i in 0..queries.len(){
            let temp: Vec<&str> = queries[i].split(":").collect();
            match temp[0]{
                "name" => {myquery.name.push(String::from(temp[1].clone()))}
                "content" => {myquery.content.push(String::from(temp[1].clone()))}
                "larger" => {myquery.larger.push(u64::from_str(temp[1].clone()).unwrap())}
                "smaller" => {myquery.smaller.push(u64::from_str(temp[1].clone()).unwrap())}
                "newer"=> {myquery.newer.push(u64::from_str(temp[1].clone()).unwrap())}
                "older"=> {myquery.older.push(u64::from_str(temp[1].clone()).unwrap())}
                _ => {println!("Invalid query found : {} : {}",temp[0],temp[1])}
            }
            // then add said query to the final result (MatchResult option)
            //let rejoined_str: &str = temp.join(":").as_str();
        }


        /// THIS DOESN'T WORK
        println!("{:?}",myquery);
        //Self::search_r(&mut self.root,&mut final_result,&myquery);

        let mut directories_to_visit = vec![&mut self.root];
        while directories_to_visit.len() > 0{
            let mut found_file = false;
            let mut current_dir = directories_to_visit.remove(0);

            for i in 0..myquery.name.len(){
                for x in 0..current_dir.children.len(){
                    if !current_dir.children[x].is_dir() && current_dir.children[x].name().eq(&myquery.name[i]){
                        println!("Found file matching name");
                    }
                    else if current_dir.children[i].is_dir(){
                        directories_to_visit.push(Node::get_mut_dir(&mut current_dir.children[x]).unwrap())
                    }
                }
            }
        }
/*

        if final_result.nodes.len() > 0{
            return Some(&mut final_result);
        }
        None

 */


        // now iterate over all the children and add items if a file matching the input is found




        /*
        for i in 0..queries.len(){
            // let result = search_function_param(...)
            // if result.is_some() {
            //      final_result.queries.push(queries[i].clone());
            //      fina_result.nodes.push(result.unwrap());
            // }
        }
         */

        /*
        TYPES OF QUERIES
        "name:string" >> find all files that have that string inside of name
        "content: string" >> find all files that contain that string inside of them
        "larger: val" >> find all files with size larger than val
        "smaller: val" >> find all files with size smaller than val
        "newer: val" >> find files created before val
        "older: val" >> find files created after val
         */
       // return None
    }
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

    let result = my_fs.get_file("Daniele/ciao/ciao.txt");

    if result.is_some(){
        println!("Found something >> {}",result.unwrap().name);
    }else{
        println!("Found nothing");
    }
    let test_queries: [&str;5] = ["name:ciao","name:test","content:saluto a mamma","newer:1","older:2"];
    my_fs.search(&test_queries);
}
