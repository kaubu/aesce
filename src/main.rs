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
}
