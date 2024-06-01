use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
pub fn run() {
    // TODO 1 - arguments
    // use std::env to print the program's first argument
    // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let dir: Vec<String> = env::args().collect();
    

    // TODO 2 - select the directory to be listed
    //  - if the first argument exists, that is the directory
    //  - if it does not, the current directory will be listed
let directory;
    if dir.len() == 1{
        directory = ".";
    }
    else{
        directory = &dir[1];
    }

    println!("{:?}", directory);

    // TODO 3 - print the contents of the directory
    // https://doc.rust-lang.org/std/fs/fn.read_dir.html
    
    // TODO 4 - print the type of each item (dir, file, link)

    // TODO 5 - print the properties of each directory / file
    // https://doc.rust-lang.org/std/fs/struct.Permissions.html#impl-PermissionsExt-for-Permissions
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    println!("{:?} directory", entry.file_name());
                    let metadata = entry.metadata().unwrap();
                    println!("{:o}", metadata.permissions().mode());
                    println!("");
                    
                }
                else if entry.path().is_file(){
                    println!("{:?} file", entry.file_name());
                    let metadata = entry.metadata().unwrap();
                    println!("{:o}", metadata.permissions().mode());
                    println!("");
                }
                else if entry.path().is_symlink(){
                    println!("{:?} link", entry.file_name());
                    let metadata = entry.metadata().unwrap();
                    println!("{:o}", metadata.permissions().mode());
                    println!("");
                }
            }
        }
    }
    // use mode
}
