
pub mod tools;
pub mod email;  // 默认是 private， 但是为了让外部也能调到，所以使用 pub 关键字，让对应的 文件的 函数能够被调用

// use crate::utils::email::utils_email_fnc;  // crate 代表当前的绝对路径， 也就是当前项目的 /src/ 路径
// 上面那个调用等价于，因为 当前的文件和 email 文件夹在同一层  
use email::utils_email_fnc; 

//  为了是当前函数能被别的包引用和 导入， 需要 添加 pub 关键字，因为 默认情况下包里面的函数全部都是私有的
pub fn utils_mod_fnc () {
    println!("this is the utils_mod_fnc !");

    utils_email_fnc();
}
