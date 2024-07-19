use std::io;

fn main() {
    println!("Temp Converter CLI");
    println!("Enter Temp to convert: ");
    let mut temp: String = String::new();
    io::stdin().read_line(&mut temp).expect("expecting a temp");

    let new_temp: i32 = temp.trim().parse().expect("expecting int");

    println!("Give a unit to convert to: (C) or (F)");
    let mut unit: String = String::new();
    io::stdin().read_line(&mut unit).expect("expecting unit");

    unit = unit.trim().to_lowercase();

    if unit == "c" {
        println!("{}°c", new_temp);
    } else if unit == "f" {
        println!("{}°f", new_temp);
    } else {
        println!("Select A Unit!");
    }
}
