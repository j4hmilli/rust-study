use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // %Scalar Types
    
    // * signed integer
    //   가장 빠른 데이터 타입
    let x: i32 = -4;

    // * unsigned integer
    let x: u32 = 4;

    // * follow the architecture type (:word)
    //   일부 컬렉션 타입의 인덱스에 사용됨
    let y: isize = 64;
    println!("{}", type_of(y));

    // * literal
    let x: i32 = 0xff; // Hex Type
    let y: i32 = 0b1111_0000; // Binary Type
    let z: u8 = b'A'; // Byte Type (`u8` only)

    // * floating
    // 기본 타입은 `f64`이며, 최신 CPU 상에서는 둘 다 비슷한 속도를
    // 내면서도 더 정밀한 표현이 가능하기 때문!
    let x = 2.0; // f64
    let y: f32 = 3.0;

    // * Boolean
    let t = true;
    let f: bool = false;

    // * charactor (Unicode Scalar)
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';


    // &Compound Types
    
    // * tuple
    let tup: (i32, f64, u8) = (500, 6.4, b'A');
    let tup = (500, 6.4, b'A');
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

}
