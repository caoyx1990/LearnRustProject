use std::net::IpAddr;

fn main() {
    println!("Hello, world!");
    // 赋值语句
    let a: u32 = 1;
    println!("{}", a);
    let a = true;
    println!("{}", a);
    let b: bool = false;
    println!("{}", b);

    // 字符
    let c = 'z';
    println!("{}", c);
    let z: char = 'ℤ';
    println!("{}", z);
    let heart_sys_cat = '😻';
    println!("{}", heart_sys_cat);
    let t = '中';
    println!("{}", t);

    // 字符串
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    // String 不能通过下标访问
    let hello = String::from("你好");
    // let a = hello[0];
    // println!("{}", a);
    /*
    error[E0277]: the type `String` cannot be indexed by `{integer}`
  --> src/001.rs:47:13
   |
47 |     let a = hello[0];
   |             ^^^^^^^^ `String` cannot be indexed by `{integer}`
   |
   = help: the trait `Index<{integer}>` is not implemented for `String`
   = help: the following other types implement trait `Index<Idx>`:
             <String as Index<RangeFull>>
             <String as Index<std::ops::Range<usize>>>
             <String as Index<RangeFrom<usize>>>
             <String as Index<RangeTo<usize>>>
             <String as Index<RangeInclusive<usize>>>
             <String as Index<RangeToInclusive<usize>>>
    */

    // 字符串字面量中的转义
    let byte_escape = "I'm saying \"Hello\"";
    println!("{}", byte_escape);
    let byte_escape = "I'm saying \n 你好";
    println!("{}", byte_escape);
    let byte_escape = "I'm saying \r\n 你好";
    println!("{}", byte_escape);
    let byte_escape = "I'm saying \\ Ok";
    println!("{}", byte_escape);
    let byte_escape = "I'm saying hello.\0";
    println!("{}", byte_escape);
    let byte_escape = "I'm saying hello \x7f";
    println!("{}", byte_escape);
    let byte_escape = "I'm saying hello \u{0065}";
    println!("{}", byte_escape);

    // 禁止转义字符串字面量
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 字节串
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    let escape = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escape);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 数组 [T; N] T 表示类型，N 表示长度
    let a: [i32; 5] = [1,2,3,4,5];
    let a = [1,2,3,4,5];
    println!("{}", a[0]);
    // println!("{}", a[5]);

    // 动态数组 Vec
    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let s1 = String::from("superman 1");
    let s2 = String::from("Superman 2");
    let s3 = String::from("Superman 3");
    let s4 = String::from("Superman 4");

    let v = vec![s1, s2, s3,s4];

    println!("{:?}", v[0]);
    // println!("{:?}", v[4]);

    // 哈希表
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores.get("Blue"));
    println!("{:?}", scores.get(&String::from("Blue")));
    println!("{:?}", scores);

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup.0);

    // 结构体
    let user = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someone@sap.com"),
        age: 1,
    };
    println!("{:?}", user);

    // 枚举
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);

    // 控制流

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, 2");
    }

    let x = 1;
    let y = if x == 0 {
        100
    } else {
        101
    };
    println!("y is {}", y);
    // 循环
    let mut counter = 0;

    let result = loop{
        counter += 1;
        if counter == 100 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is : {element}");
    }

    for number in 1..4 {
        println!("{number}");
    }

    println!("---");

    for number in 1..=4 {
        println!("{number}");
    }

    println!("---");

    for number in (1..4).rev() {
        println!("{number}");
    }

    for ch in 'a'..='z' {
        println!("{ch}");
    }
}



#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    age: u64
}
