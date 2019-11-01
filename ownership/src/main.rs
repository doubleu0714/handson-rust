fn main() {
    println!("Hello, world!");

    let s1 = "Hello"; // Immutable
    println!("s1: {}", s1);

    let mut s = String::from("Hello");
    s.push_str(", world!!");
    println!("s: {}", s);

    let move1 = String::from("Hello");
    let move2 = move1;
    // Heap에 있는 데이터를 참조하는 포인터가 이동하면 기존 포이터를 가지고있는 변수는 사용할 수 없음
    // println!("move1: {}", move1);

    let deep1 = String::from("Deep");
    let deep2 = deep1.clone();
    
    let arg1 = String::from("ARG");
    printString(arg1); // arg1의 소유권이 printString 함수로 move 되었다.
    // println!("{}", arg1); // 따라서 더이상 유효한 변수가 아님.

    let arg2 = 5;
    printInt(arg2); // arg2는 stack에 저장되는 값이므로 move가 아닌 copy가 된다.

    let arg3 = String::from("ARG");
    printStringReference(&arg3); // Reference를 인자값으로 넘기면 ownership이 move하지 않는다.
    println!("{}", arg3); // 따라서 계속해서 변수를 사용할 수 있음

    let mut arg4 = String::from("ARG");
    let refer1Arg4 = &arg4;
    let refer2Arg4 = &arg4;
    println!("{}, {}", refer1Arg4, refer2Arg4);
    let refer3Arg4 = &mut arg4;
    // let refer4Arg4 = &mut arg4;
    // println!("{}, {}, {}, {}", refer1Arg4, refer2Arg4, refer3Arg4, refer4Arg4);
    // println!("{}", refer1Arg4);
    println!("{}", refer3Arg4);
    printStringReference(refer3Arg4);
}

fn printString(arg: String) {
    println!("{}", arg);
}

fn printStringReference(arg: &String) {
    println!("{}", arg);
}

fn printInt(arg: i32) {
    println!("{}", arg);
}
