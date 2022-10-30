use std::fs;
use std::io;

fn main() {
    let path = String::from("hello.txt");
    let user_name = read_username_from_file(&path);

    match user_name {
        Ok(user) => println!("user name: {}", user),
        Err(e) => println!("can't read file from the file <{}>. Error: {}", &path, e)
    }

}

fn read_username_from_file(path: &String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
