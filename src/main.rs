use std::collections::HashMap; // 调用标准库

fn main() {
    //  创建一个 hash map
    let mut scores: HashMap<String, i32> = HashMap::new();
    //  插入键值对
    //  注意 insert 的插入，如果原键值对不存在，就插入，如果存在，就直接覆盖掉，而不会报错
    scores.insert(String::from("Alice"), 50);
    scores.insert(String::from("Bob"), 70);

    println!("scores map: {scores:?}"); // 打印： scores map: {"Alice": 50, "Bob": 70}

    // 访问键值
    let alice_score: Option<&i32> = scores.get("Alice"); // Option 查询得到就返回对应的值，查不到就会得到一个 None
    println!("alice_score : {alice_score:?}"); // 打印： alice_score : Some(50)
    // 1. 使用 let 语法单独解析
    if alice_score.is_some() {
        // 这里面还可以这么写
        // 下面这是 Rust 1.65+ 引入的「let-else 语法」
        //  但是在实际使用时个人不推荐，因为 下面的 return 行为 会直接从整个 当前的 main 函数退出，导致后面的逻辑不会被执行
        //  但是由于当前 的 if 里面加了 alice_score.is_some() 判断，这从而避免的了 如果查询结果是 None 时，直接从当前 main 函数 直接 return 退出
        let Some(score) = alice_score else { return; };

        /*
          其上上面等价于 下面这段老版本代码，只是看起来有点啰嗦
          let score = match alice_score {
                Some(s) => s,
                None => return,
            };
        */ 

        println!("Alice's score 1: {}", score); // 打印： Alice's score 1: 50
    }

    // 2.直接在 if 条件判断中使用 let - else 语法
    if let Some(score) = alice_score {   
        println!("Alice's score 2: {score}"); // 打印： Alice's score 1: 50
    }

    //  使用match
    let scr = match alice_score {
        Some(sc) => sc,
        None => &-1  // 假设如果没能查询到，直接返回 -1，这里只是借用 -1而已
    };
    println!("Alice's score 3 : {scr}"); // 打印： Alice's score 3 : 50 ， 如果以上 alice_score 没能正常获取到， 此时能正常打印到 Alice's score 3 : -1


    // 下面这行代码表示： 如果能找到，就返回指定的值，如果没找到就返回 -1， 其实和上面我的代码基本类似，只是比我代码简洁一些
    // copied() 方法，就是取出值的拷贝，也就是将 get 方法得到的 Option<&i32> 中的值取出来
    // 注意： 如果 scores.get("Bertram") 不是 Copy 类型，比如 String，copied() 就不能用了，需要 .cloned()
    let sre = scores.get("Bertram").copied().unwrap_or(-1);
    /*
     上面就等级于下面代码：
     let sre = match scores.get("Bertram") {
        Some(&v) => v,   // 取出值的拷贝
        None => &-1,      // 没找到就用默认值 -1
     };
    
    */ 
    println!("the Bertram score is {sre}"); // 打印： the Bertram score is -1
    

    //  还可以直接一键获取修改
    // 使用 get_mut 获取 "Alice" 的可变引用
    if let Some(alice_score) = scores.get_mut("Alice") {
        *alice_score += 10; // 解引用后直接修改
    }



    // 遍历, 注意下面只是借用 &scores，没有改变 ownership
    /* 
        以下便利打印：
        Bob: 70
        Alice: 60  // 因为上面手动 利用 get_mut 进行了修改
    */
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }



    // 删除键值对 , 删除键 "Bob"
    println!("before delete: {scores:?}");  // 打印： before delete: {"Bob": 70, "Alice": 60}
    if let Some(value) = scores.remove("Bob") {
        println!("deleted value:  {value}");  // deleted value:  70
    } else {
        println!("Bob score not existed !"); 
    }
    println!("after delete: {scores:?}"); // after delete: {"Alice": 60}
}
