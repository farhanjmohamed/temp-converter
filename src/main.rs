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
        let converted_c = (new_temp - 32) * 5 / 9;
        println!("{}°C", converted_c);
    } else if unit == "f" {
        let converted_f = (new_temp * 9 / 5) + 32;
        println!("{}°F", converted_f);
    } else {
        println!("Select A Unit!");
    }
}
