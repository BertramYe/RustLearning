
fn main() {   
    let mut x = 5;
    let p: *mut i32 = &mut x;  // 可变裸指针
    unsafe {
        *p = 10;  // 通过裸指针修改数据
    };
    unsafe  {
        println!("the value of the p points to: {}",*p); // 解引用裸指针，打印它指向的值
    }
}





