// 상수 선언은 const 키워드가 필요하고, 반드시 데이터 타입을 명시해야한다.
const GLOBAL_CONST: u32 = 150;
fn main() {
    // Immutable을 변수 값 변경이 불가능하다. 값 변경 시도시 컴파일 에러
    let x = 5;
    // x = 10; // 컴파일 에러
    println!("The value of x is : {}", x);
    
    // Mutable 변수로 선언하기 위해서는 mut 키워드를 사용한다.
    let mut y = 100;
    println!("The value of y is : {}", y);
    y = 500;
    const LOCAL_CONST: u32 = 123;
    println!("The value of y is : {}", y);
    println!("The value of LOCAL_CONST is : {}", LOCAL_CONST);
    println!("The value of GLOBAL_CONST is : {}", GLOBAL_CONST);

    // shadowing 으로 동일한 변수명으로 선언할 수 있다.
    let z = 100;
    println!("The value of z is : {}", z);
    let z = z * 2;
    println!("The value of z is : {}", z);
    let z = z * 2;
    println!("The value of z is : {}", z);

    // integer types
    let signed : i8 = 123;
    let default_integer = 123; // i32
    println!("The value of signed is : {}", signed);
    // overflow 발생시 런타임 에러
    // let overflow : i8 = z;
    let char_val: char = '박'; // unicode이므로 한글도 가능
    println!("The value of charVal is : {}", char_val);

    // Tuple
    let tup = (13, '가', 15.01);
    // destructuring
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("0 = {}, 1 = {}, 2 = {}", tup.0, tup.1, tup.2);

    // Array
    let array1 = [1, 2, 3, 4, 5];
    // let array2: [char; 3] = ['r', '가', '나', '다']; // fixed length이기때문에 이렇게 할당 할 수 없음.
    let array2: [char; 4] = ['r', '가', '나', '다'];
    // println!("array1[10] = {}", array1[10]); // 유효하지 않은 인덱스로 접근하면 컴파일 오류가 발생

    // sum
    println!("sum {}", sum(1, 2));
    let testVal = 15;
    let expressionVal = {
        let testVal = 3;
        testVal + 1
    };
    println!("testVal : {}, expressionVal : {}", testVal, expressionVal);
}

fn sum(param1: i32, param2: i32) -> i32 {
    param1 + param2
}