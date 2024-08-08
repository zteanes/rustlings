#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership; so we change it to a pointer
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership, so we give it normal string
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // change to give pointer

    string_uppercase(data); // changer to give string and give ownership
}
