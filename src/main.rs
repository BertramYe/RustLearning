fn main() {
   // 故此对于字符串，为了解决中英文混合的问题，可以借助 chars 来帮助实现按照
   // chars()：将字符串转换为一个字符迭代器，每次迭代返回一个 char，即一个 Unicode 字符，同时每个 char 大小为 4 字节，足够容纳单个中文的 3 字节长度，这样就完美解决了中英文混用的切片不准确的问题。
    let chinese_string:&str = "Hello 你好 Rust 世界！";
    // 获取前 6 个字符（不按字节）
    let substring_1: String = chinese_string.chars().take(6).collect(); // 默认从 chars 的索引 0 开始，往后面取 6 位
    println!("substring_1: {substring_1:?}"); // 打印结果为： substring_1: "Hello "
    // 获取字符索引范围从 7 到 14 (即 6 + 9 - 1) 的字符串， 也就是是，首先跳过前 0~5 位字符串 （字符索引从 第 6 位开始），开始往后面数 9 位字符串
    let substring_2: String = chinese_string.chars().skip(6).take(9).collect();
    println!("substring_2: {substring_2:?}"); // 打印结果为： substring_2: "你好 Rust 世"
 

}


