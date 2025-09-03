use std::env;
use std::fs;

use std::path::Path;

fn main() {

println!("\nThis is mini-git vcs!\n");

let  vcs_args: Vec<String> = env::args().skip(0).collect();


let command = &vcs_args[1];

    if command == "init" {
    println!("Messge: {} initialized!\n", command);

    let create_folder_name = ".minigit";

    let folder_path = Path::new(create_folder_name);

    // let abs_path = fs::canonicalize(folder_path).expect("Could not find the absolute path for the initialized mini-git repo");

    if !folder_path.exists() {
        
        fs::create_dir(create_folder_name).expect("Folder not created.");
        println!("Initialized empty mini-git repository in: {}\n",
        fs::canonicalize(folder_path).expect("Could not find the absolute path for the initialized mini-git repo").display());
    
    } else {
        
        println!("Mini-git repo already exists at path: {}\n", 
        fs::canonicalize(folder_path).expect("Could not find the absolute path for the initialized mini-git repo").display());
      
    }


 } else if command == "add" {

    println!("Message: {} command is running!", command);

    let add_args: Vec<String> = env::args().skip(2).collect();
    
    // let rest_args = &add_args[2..];

    if add_args.len() == 0 {
        println!("No file names present!");
    }

   for file in add_args {
    let path = Path::new(&file);
    if path.exists() {
        println!("✅{} exists!", file);
    } else {
        println!("❌{} does not exist!", file);
    }
   }

 } else {
     println!("Unknown command: {}", command);
 }



 } 


