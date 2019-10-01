// io 라이브러리를 소스 scope로 가져온다.io 라이브러리는 std라이브러리에서 가져온다.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// main function이 entry point이다.
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");

        // 변수 선언 키워드: let
        // Rust에서 변수는 기본적으로 immutable 이다.
        let mut guess = String::new(); // ::new 는 associated function이다. 타입의 새로운 값을 생성할떄 사용함.

        // std::io::stdin 으로도 사용할 수 있으나 상단에 use절을 사용했으므로 "std"는 생략 가능하다.
        // &의 똣은 포인터와 같다. 즉 변수의 reference를 뜻한다.
        // reference 도 기본적으로 immutable이기때문에 reference의 값을 변경하기 위해서 &mut이라고 한다.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // 똑같은 이름의 guess 변수 사용 : Shadow 기법, 주로 형변환을 위해 별도 변수(str_guess, int_guess등)를 할당하는 비효율을 없애기 위해 사용
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");
        // 아래와 같이 match 표현식으로도 가능하다. Result 가 enum이기 때문에
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
