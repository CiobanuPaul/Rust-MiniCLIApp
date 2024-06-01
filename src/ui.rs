use cursive::{self, Cursive};
use cursive::views::{TextView, SelectView, EditView, DummyView, Dialog, LinearLayout, ScrollView};
use cursive::traits::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
pub fn run() {
    // TODO 1 - tutorial
    // follow all 3 turorials https://github.com/gyscos/cursive/blob/main/doc/tutorial_1.md
    let mut siv = cursive::default();
	siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('b', |s| {s.pop_layer();});
    let mut select = SelectView::<String>::new()
        .on_submit(on_submit);
    getDir(&mut select, ".");
    siv.add_layer(Dialog::around(select).title("Select a file/directory"));

    siv.run();

    
}


fn on_submit(s: &mut Cursive, path: &str){
    
    let mut select = SelectView::<String>::new()
        .on_submit(on_submit);
    let path_buf = PathBuf::from(path);
    let is_dir = path_buf.is_dir();
    let is_file = path_buf.is_file();
    if is_dir {
        s.pop_layer();
        getDir(&mut select, path);
        s.add_layer(Dialog::around(select).title("Select a file/directory"));
    }
    else if is_file{
        let contents = fs::read_to_string(path)
                .expect("Should have been able to read the file");
        s.add_layer(ScrollView::new( Dialog::text(contents)));
    }
}
// fn ok(s: &mut Cursive, name: &str){
//     s.call_on_name("select", |view: &mut SelectView<String>|{
//         view.add_item_str(name);
//     });
// }

fn getDir(s: &mut SelectView, dir: &str){
    if let Ok(entries) = fs::read_dir(dir) {
        let mut dir_clone = String::from(dir.clone());
        dir_clone.push_str("/..");
        // println!("{:?}", dir_clone);
        s.add_item("parent directory", dir_clone.into());
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                let file_path = entry.path().into_os_string().into_string().unwrap();
                s.add_item(file_name, file_path);
                // if entry.path().is_dir() {
                //     println!("{:?} directory", entry.file_name());
                //     let metadata = entry.metadata().unwrap();
                //     println!("{:o}", metadata.permissions().mode());
                //     println!("");
                    
                // }
                // else if entry.path().is_file(){
                //     println!("{:?} file", entry.file_name());
                //     let metadata = entry.metadata().unwrap();
                //     println!("{:o}", metadata.permissions().mode());
                //     println!("");
                // }
                // else if entry.path().is_symlink(){
                //     println!("{:?} link", entry.file_name());
                //     let metadata = entry.metadata().unwrap();
                //     println!("{:o}", metadata.permissions().mode());
                //     println!("");
                // }
            }
        }
    }
}