

struct  Color(u8,u8,u8);
struct Point(f32,f32);

fn main() {
    // 初始化以上的对象
    let red = Color(255, 0, 0);
    let p = Point(10.5, 20.3);

    // 访问字段：通过 .0、.1、.2 下标
    // 这里使用占位符，是因为不可以使用 {red.0} 这种写法，会报错的
    println!("Red = ({}, {}, {})", red.0, red.1, red.2); // 打印： Red = (255, 0, 0)
    println!("Point = ({}, {})", p.0, p.1); // 打印： Point = (10.5, 20.3)

}
