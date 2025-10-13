
fn main() {
    let a: &str = "hhhhh";

    //  注意下面是 rust 的一个代码块（code block）, 它回像普通代码一样被执行，只是不同点在于作用域的问题
    {
        //  这一步会 先拿到 原来的 a 的值进行拼接，接着会将上面的 a 进行遮蔽，在当前 代码块的作用域中使用新的 a
        let a = a.to_string() + "aaa"; 
        println!("the inner a value is {a}"); // 会打印 ： the inner a value is hhhhhaaa
    }

    // 这一步会先拿到原来 a （即 hhhhh ）的长度 a.len() ， 在将其值进行遮蔽，从这里开始，用新的 a 来代替
    let a: usize = a.len();
    let a: usize = a * 2;

    println!("the outer value of  a is {a}"); // 会打印： the outer value of  a is 10

}


