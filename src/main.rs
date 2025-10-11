



fn main() {
    let a = "a";
    #[allow(unused_assignments)] // 注释掉未使用变量的编译报警提示
    let mut  b = "b";  // 位置接使用而在后续重新赋值，可能会有 unused_assignments 报警，此时可以使用下划线跳过未使用变量的提醒,或者使用 #[allow(unused_assignments)] 注释掉未使用变量的编译报警
    b = "B";
    // 虽然 const 定义的 常量可以使用小写的 c 表示，而在实际的编译时也只是报警，但是当在真正使用时，最好使用大写定义常量
    // const c:&str = "c";
    // 虽然以上不报错，但是在以下使用时，最好使用大写 C 来表示
    const C:&str = "c";

    println!("this is the value a: {a},b: {b} , c: {C}"); // 打印： this is the value a: a,b: B , c: c
    // 利用 const 定义的常量可以作用于全局
    println!("the static value TEST_CONST_VALUE is : {TEST_CONST_VALUE}"); // 打印： the static value TEST_CONST_VALUE is : hhhhh
    println!("The static AAA is {AAA}"); // 打印： The static AAA is aaa
    // 使用 unsafe 来访问 _BBB
    // unsafe {
    //     println!("The static BBB is {_BBB}"); // 会报错，因为可变的静态变量太危险
    // }
}

// 使用 const 定义的常量可以作用于全局，同时const 定义的常量一定要标注其常量类型，否则在Rust中编译时，一定报错
const TEST_CONST_VALUE:&str = "hhhhh"; 
//  其实上面这种使用 cosnt 定义的常量，和下面这种利用 static 定义的常量有点类似，都可以作用于全局，但是其存储的地方和生效是不一样的
static AAA:&str= "aaa";
static mut _BBB:&str= "BBBB"; // 虽然可以定义可变的静态变量，但是由于太危险，所以上面的调用会报错
