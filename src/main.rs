fn main() {
    let mut  counter = 0;
    while counter < 10 {
        counter += 1;
        if counter % 2 == 0 {
            continue;  // 跳过偶数，不执行后面的逻辑，而重新进行下一轮循环
        }
        // 偶数不执行这一行逻辑
        println!("odd number: {counter}");  
    }

}