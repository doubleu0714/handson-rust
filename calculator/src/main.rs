use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let a1: i32 = args[1].parse::<i32>().unwrap();
    let operator: &String = &args[2];
    let a2: i32 = args[3].parse::<i32>().unwrap();
    let result: i32 = calculate(a1, operator, a2).unwrap();
    println!("{} {} {} = {}", a1, operator, a2, result);
}

fn calculate(a1: i32, operator: &String, a2: i32) -> Result<i32, &str> {
    if operator == "+" {
        Ok(a1 + a2)
    } else if operator == "*" {
        Ok(a1 * a2)
    } else if operator == "-" {
        Ok(a1 - a2)
    } else if operator == "/" {
        Ok(a1 / a2)
    } else {
        Err("not exist operator")
    }
}
