fn main() {
    let height = human_inform("Bertram",18,175.577);
    print!("final get height is {height}")

    // 以上的运行结果如下：
    // hello , my name is Bertram, I am 18 years old, and my height is 175.58 cm.
    // final get height is 175.577
}

fn  human_inform(name:&str,age:i32,height:f32) -> f32 {
    // 其中 {height:.2} 表示保留两位小数,同时它会自动进行四舍五入；
    println!("hello , my name is {name}, I am {age} years old, and my height is {height:.2} cm.");
    // 可以使用 return 关键字直接 return 结果 
    // return height;  // 本质可以理解为一个简单的 statement，结尾必须携带 分号
    // 也可以不使用 return关键字，直接这么写，也可以返回结果，但是需要注意的是，下面这种写法，不能使用 分号结尾
    height  // 本质可以理解为一个简单的 expression 因为其自身作为 return 的返回结果，所以结尾不能携带分号

    // 这是因为对于以上的结果，需要理解两个概念
    // 1. expression： 所有有返回值的代码，可以称为表达式
    // 2. statement: 所有没有返回值的代码，可以称为语句
    // 在 rust 中所有的几乎所有的 statement 的结尾必须携带 分号
}