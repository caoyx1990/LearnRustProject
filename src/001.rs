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

}
