# handson-rust
* 변수는 기본적으로 Immutable이다.
  - 변수 선언 keyword : let variable
  - 변수 reference를 넘길 때에는 '&' 사용 : &variable
  - mutable 변수, reference 를 만들 때에는 mut keyword 사용 : let mut variable, &mut variable
  - shadow 기법 : 동일한 로컬 변수명을 사용할 수 있음. 형변환시 별도의 변수를 선언하지 않아도 됨
    + ``` rust
        let guess = 선언문;
        let guess = 형변환;
      ```
* enum 타입에 대한 분기는 match expression으로도 가능하다.
  - ``` rust
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
    ```
* Cargo.lock파일이 존재한다면 해당 파일에 명시되어잇는 버전을 사용합니다. 그렇지 않다면 Cargo.toml파일로 빌드를 실행합니다.
