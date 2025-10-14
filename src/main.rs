// src/main.rs
mod same_level; // 引用同级别目录下面的 模块文件,即默认相对路径不用 . 表示
mod utils;

// 其中 crate 代表根目录,这是绝对路径的表示
use crate::utils::utils_mod_fnc;



fn main() {
    // 调用同级别目录下面的 包里面的函数
    same_level::same_level_fnc(); // 打印： this is the same_level_fnc !

    // 调用 utils 文件下的 mod.rs 文件 里面的 utils_mod_fnc 函数
    // utils::utils_mod_fnc();
    // 由于使用了 use crate::utils::utils_mod_fnc; 引用对应函数，从而可以像下面这样调用
    // 否则依托于 mod utils; 引用，可以直接像上面一样使用 utils::utils_mod_fnc(); 去调用
    utils_mod_fnc();

    // 调用 utils 文件下的 tools 文件夹下的 mod.rs 文件里面的 utils_mod_fnc 函数
    utils::tools::utils_tools_fnc();
    
}
