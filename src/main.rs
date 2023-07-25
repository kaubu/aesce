mod fs;

fn main() {
    println!("Hello, world!");

    let file = fs::File::new(
        "/home/tom".to_string(),
        "emails.txt".to_string()
    );

    println!("{file:?}");
}
