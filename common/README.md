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
