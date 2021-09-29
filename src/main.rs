use std::collections::HashMap;
use std::io;

mod counter1k_7;
mod bytes2human;

fn main() {
    let mut apps  = HashMap::new();
    let mut input = String::new();
    let mut known_methods = String::new().to_owned();

    apps.insert("counter1k_7", counter1k_7::calculate as fn());
    apps.insert("bytes2human", bytes2human::convert as fn());

    for (key, _value) in &apps {
        known_methods.push_str(&*format!("{}, ", key))
    }

    println!("[INFO] List of Projects: {}", known_methods);
    println!("[INFO] Choose project from the list above: ");

    io::stdin().read_line(&mut input).unwrap();

    match apps.get(input.trim()) {
        Some(project) => project(),
        None => println!("[Error] Unknown project!")
    }
}
