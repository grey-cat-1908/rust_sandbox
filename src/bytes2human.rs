use std::{io};

pub(crate) fn convert() {
    let mut input = String::new();


    println!("Введите число байтов: ");
    io::stdin().read_line(&mut input).unwrap();

    let num = input.trim().parse::<i64>().unwrap();
    let units = vec!["kB", "MB", "GB"];

    for item in units.iter() {

        let item_position = units.iter().position(|r| r == item).unwrap();

        if (num as i64) < 1024 {
            println!("{}B", num);
            return;
        }

        let number = num / (1024u32.pow((item_position as i32 + 1) as u32) as i64);

        if number >= 1024 {
            // pass
        }
        else {
            println!("{}{}", number, item);
            return;
        }
    }
    println!("копец как много, хватит, перестань!")
}