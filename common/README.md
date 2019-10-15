# Common Concepts
## 변수
&nbsp;Rust의 변수는 기본적으로 Immutable이다. 이유는 rust가 제공하는 안전하고 쉬운 동시성에 대한 이점을 가져갈 수 있게 하기 위함이다. 아래의 코드는 컴파일 에러가 발생한다.  
``` rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```
&nbsp;변수를 Mutable로 만들기 위해서는 `mut` 키워드만 붙이면 된다.  
``` rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```
&nbsp;변수와 상수의 차이
* `mut` 키워드를 상수에는 붙일 수 없다.
* `const` 키워드를 사용해 상수를 선언 할 때에는 type을 반드시 명시해야한다.
* 상수는 어떠한 scope에도 선안될 수 있다
* 상수에 값 할당은 함수의 결과나 runtime시 결정되는 값으로는 할 수 없다.
&nbsp;shadowing이란 같은 scope내에서 동일한 변수명으로 선언하는 것을 말한다.  
``` rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```  
``` rust
let spaces = "   ";
let spaces = spaces.len();
```
&nbsp;shadwing과 mutable변수는 다른것이다. shadowing 이후 불변성을 갖지만, mut는 계속해서 가변성을 가지고 있다. 그리고 shadowing은 변수의 이름만 같은 새롭게 정의된 다른 type의 변수이지만 mut는 동일한 변수 type이다. 아래의 코드는 컴파일 에러가 발생한다.
``` rust
let mut spaces = "   ";
spaces = spaces.len();
```
## Data types
&nbsp;Rust의 모든 값들은 데이터 타입이 고정적으로 정해져 있으며, 이는 compile time에 변수의 타입을 확실히 알고 있어야 한다는 의미이다.  
&nbsp;데이터 타입의 종류
* Scalar types
  - Integre types
    + 정수를 표현하는 타입
    + {signed|unsigned}{length} 로 표현된다.
    + signed : i, unsigned u 로 표현한다.
    + length : 8, 16, 32, 64, 128, arch(컴퓨터의 아키텍쳐에 의존 32bit or 64bit)
    + 기본타입은 i32 이다.
    + example : i8(-128 ~ 127), u8(0 ~ 255)
    + integer literals
      - Decimal : 90_222
      - Hex : 0xff
      - Octal : 0o77
      - Binary : 0b1111_0000
      - Byte(u8 only) : b'A'
    + overflow 발생시 runtime error 발생
      - `RUST_BACKTRACE=1 cargo run` 실행시 `src/libstd/panicking.rs` 에서 에러가 발생됨이 보인다.
  - Floating-Point types
    + 실수를 표현하는 타입
    + size 별로 f32, f64가 있고 기본타입은 f64이다.
  - 산수 연산자 : +, -, *, /, % 
  - Boolean Type
    + `true` or `false` 값을 가지며 키워드는 `bool`이다.
  - Charactor Type
    + `char` 타입은 작은 따옴표를 사용한다.
    + Unicode scalar를 표현하는 값이며 이는 ASCII보다 많은 표현을 가능하게 합니다.
* Compound Types
  - Tuple type
    + 튜플은 다양한 유형의 몇 가지 값을 하나의 유형으로 그룹화 하는 것을 말한다.
    + 고정된 길이이며 줄이거나 늘리지 못한다.
    + ``` rust
      fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
      }
      ```
  - Array type
    + Tuple과는 다르게 원소들이 모두 동일한 data type을 가져야 한다.
    + 고정길이이다.
    + Heap이 아닌 stack 에 할당된다.
    
## Function
&nbsp;함수 시그니처는 `fn function_name([param_name: param_type]*) [-> return_type]` 이다.  
&nbsp;함수 body는 statements와 expressions로 이루어져 있다.
* Statements : 어떠한 행동을 수행하도록 하는 것이고 return이 없다. 실행가능한 최소의 독립적인 코드. 여러개의 expression과 프로그래밍 키워드를 포함.
* Expressions : 결과를 나타낼 수 있는 단위. 하나의 값으로 표현될 수 있는 코드
* ``` rust
  fn main() {
      let x = 5; // statement

      let y = { // expression단위는 { expresion }
          let x = 3;
          x + 1 // expression의 마지막 결과 리턴은 ;를 붙이지 않는다.
      };

      println!("The value of y is: {}", y);
  }
  ```
&nbsp;함수의 리턴은 `return` 키워드를 사용하기도 하지만 대부분의 함수는 마지막 expression을 암묵적으로 리턴한다. 

## Control Flow
* if
  * ``` rust
    fn main() {
      let number = 3;

      if number < 5 {
          println!("condition was true");
      } else {
          println!("condition was false");
      }
      // 아래처럼 let statement와 함께 사용가능
      // 단, expression의 리턴 타입이 동일해야함
      let number = if condition {
          5
      } else {
          6
      };
    }
    ```
* loop
  * ``` rust
    fn main() {
        loop {
            println!("again!");
        }

        let mut counter = 0;
        // let statment와 함께 사용 가능
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };
    }
    ```
* while
  * ``` rust
    fn main() {
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }
    ```
* for
  * ``` rust
    fn main() {
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }
    ```