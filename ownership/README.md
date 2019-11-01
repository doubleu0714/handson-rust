# Ownership
ownership이란 rust가 가비지 콜렉터 없이 메모리 safe를 보장할 수 있는 유니크한 기능입니다. 

모든 프로그램은 사용하는 메모리에 대한 관리가 필요합니다. 어떠한 시스템은 가비지 콜렉터를 사용해 지속적으로 사용되지 않는 메모리를 검사하기도 하고, 개발자가 명시적으로 메모리를 해제 하도록 설계된 시스템도 있습니다. 

rust는 다른 방법을 사용하는데 컴파일 타임에 룰셋으로 컴파일러가 체크 하도록 하는 ownership시스템을 사용합니다.  

> stack & heap  
Stack과 Heap은 메모리의 영역의 일부분이며 다른방식으로 구현되어있습니다.  
Stack은 LIFO구조로 컴파일타임에 사이즈를 알고있는 고정길이 데이터들이 쌓이는 영역이지만, Heap은 컴파일 타임에 길이를 알 수 없고, 가변길이인 데이터들이 저장되는 영역입니다. 데이터를 Heap 영역에 할당(allocate)하려면 OS는 Heap영역에서 충분한 공간을 확보하고 그 공간의 Pointer를 반환해주게 됩니다. 따라서 Stack에 push 하는 것 보다 느릴 수 밖에 없습니다. 왜냐하면 Stack은 단순히 push만 해주면 되기 때문입니다.(고정길이이기 때문). 그리고 데이터에 접근하는 것도 Heap이 더 느립니다. 왜냐하면 포인터를 따라가야 하기 때문입니다.  

Ownership은 어떠한 코드가 Heap영역에 데이터를 저장하는지 감시하고, Heap영역의 중복된 데이터를 최소화하고, 사용되지 않는 데이터를 Heap 영역에서 정리하는하는 역할을 합니다.  

## Ownership Rules
* Rust 내의 모든 값은 *Owner*라 불리는 변수를 가지고 있다.
* 한번에 하나의 *Owner*만 존재한다.
* *Owner*가 scope를 벗어나면, 그 값은 버려진다.  

### Scope란?
``` rust
fn main() {
    {
        // s는 유효하지 않습니다. 아직 선언이 안됐거든요.
        let s = "hello";// s는 이 지점부터 유효합니다.
        // s를 가지고 뭔가 합니다.
    }
    // 이 스코프는 이제 끝이므로, s는 더이상 유효하지 않습니다.
}
```
변수의 Scope는 다른 언어와 비슷하다.  

### String Type
고정길이 Datatype들은 Stack에 저장(Scope 시작에 Stack에 push, Scope를 벗어날 때 pop) 되므로 ownership과 큰 연관이 없다. 우리는 가변길이 Data type의 대표적인 예인 String을 살펴봐야 한다.  
String literal의 경우는 최종 실행파일에 하드코딩되어 들어가게 되므로 빠르고 효율적이지만, 이러한 속성은 String literal이 immutable이기 때문입니다. 우리는 컴파일 타임에는 사이즈를 알 수없는 가변적인 문자열을 메모리에 넣어야 합니다. 컴파일타임에는 사이즈를 알 수 없는 가변적인 문자열을 Heap에 저장하고 내용을 가지고 있으야 합니다. 그렇게 하기 위해서는
* 할당되어야 하는 메모리는 반드시 Runtime에 OS로부터 요청되어야합니다.
  - ```String::from``` 함수
* 우리가 String에 대한 작업이 끝나면 메모리를 OS에 반환하는 방법이 필요합니다.
  - 가비지 콜렉터를 사용하는 언어에서는 GC가 필요없는 데이터를 언젠가는 수집해 메모리를 확보하게 됩니다.
  - 명시적으로 메모리를 해제(free)하는 언어도 있습니다.
  - Rust는 변수가 Scope밖으로 벗어나면  메모리가 자동(Rust에서 Drop함수를 호출)으로 반환됩니다. 
    ``` rust
    #![allow(unused_variables)]
    fn main() {
        {
            let s = String::from("hello"); // s is valid from this point forward
            // do stuff with s
        }
        // this scope is now over, and s is no longer valid
    }
    ```

### move
다른 프로그래밍방식에는 shallow copy, deep copy라는 것이 있습니다. rust에서는 shallow copy의 개념이 move라는 개념으로 변형됩니다.  
``` rust
let move1 = String::from("Hello");
let move2 = move1; // move1의 포인터가 move2로 move됨. 
// move1은 더이상 유효한 변수가 아님
```
Stack영역에 저장되는 데이터는 move가 아닌 copy가 된다.  
* u32와 같은 모든 정수형 타입들
* true와 false값을 갖는 부울린 타입 bool
* f64와 같은 모든 부동 소수점 타입들
* char 타입
* Copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 Copy가 되지만, (i32, String)은 안됩니다.  

deep copy는 ```clone``` 함수를 호출해서 사용할 수 있으며, 힙에 저장된 데이터가 복사되고 각각 변수는 다른 메모리 주소를 참조하게 된다.
``` rust
let deep1 = String::from("Deep");
let deep2 = deep1.clone();
```

## 참조자
함수 내부의 로컬변수의 ownership을 그대로 유지하면서 다른 function의 인자로 넘겨줄 수 있도록 해줄는 기법. 변수명 앞에 '&'를 붙이면 해당 변수의 참조자를 의미하며 function 선언시 인자 타입에도 '&'를 붙여야 한다.  
``` rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 가 scope를 벗어났지만 참조자이기 때문에 ownership이 없다 따라서 아무것도 하지 않는다.
```  

참조자도 기본적으로 immutable이다 따라서 borrowing한 값을 수정하면 컴파일 오류가 발생한다. 이를 방지하기 위해서는 'mut' 키워드를 붙여야 한다.  
``` rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
mutable 참조자는 큰 제약 조건이 하나있다. 단위 scope 내에서 하나의 변수에는 하나의 mutable 참조자만 생성하는 것이 가능하다.

## Slice Type
slice type 도 ownership을 가지지 않는다.