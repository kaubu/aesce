use crate::fs::File;

mod fs;

// Consts
const ROOT_DIRECTORY: &str = "./";
const FILES_DIRECTORY: &str = "./files";

fn main() {
    println!("Hello, world!");

    let file = fs::File::new(
        "/home/tom".to_string(),
        "emails.txt".to_string()
    );

    // println!("{file:?}");

    let mut root = fs::Directory::create_root_dir();

    root.add_file(
        File::new("/".to_owned(), String::from("test.txt"))
    );

    let path = fs::Directory::get_path(
        &root, Vec::new()
    );

    println!("Path = {path}");
}
