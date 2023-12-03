use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let a1: i32 = args[1].parse::<i32>().unwrap();
    let operator: &String = &args[2];
    let a2: i32 = args[3].parse::<i32>().unwrap();
    let result: i32 = if operator == "+" {
        a1 + a2
    } else if operator == "*" {
        a1 * a2
    } else if operator == "-" {
        a1 - a2
    } else if operator == "/" {
        a1 / a2
    } else {
        panic!("not exist operator")
    };
    println!("{} {} {} = {}", a1, operator, a2, result);
}
