trait Barking {
    fn bark(&self); // 特征 Barking 抽象出了一个 bark 方法
}

struct Dog {
    name: String,
}

struct Engine {
    horsepower: u32,
}

struct Car {
    maker: String,
    engine: Engine,
    barker: Box<dyn Barking>,  // 组合并实现多态
}

// 为 Dog实现 Barking 特征中的 bark 方法
impl Barking for Dog {
    fn bark(&self) {
        println!("{} says Woof!", self.name);
    }
}

impl Car {
    // 没有 self 的 静态方法
    fn new(make: String, horsepower: u32, barker: Box<dyn Barking>) -> Car {
        
        // 由于此时下面省略了关键字 return， 所以结尾一定不能添加分号 ； ， 
        // 否则会被 Rust 看成是 普通语句，并返回 (), 注意此时返回的 () 不是tuple，而是单元类型，而不会返回 Car 实例化的对象，从而报错。
        Car {
            maker:make,
            engine: Engine { horsepower },
            barker,  // 这里可以像js 一样，往对象里面传参
        } 
    }

    // 这是实例化方法，有 &self 入参
    fn play_sound(&self) {
        println!("{} maked the {}'s housepower car",self.maker,self.engine.horsepower);
        self.barker.bark();
    }
}

fn main() {
    let dog = Dog { name: String::from("Buddy") };
    // 先狗吠
    dog.bark(); // 输出 Buddy says Woof!
    // 注意对于下面的 Car 由于在实例化时，直接入参了 dog 对象，
    // 所以在下面的 函数执行完时，dog 会被drop掉，这是因为 下面的new在调用时,入参dog 会导致 dog 的所有权ownership发生变动,所以会被销毁。 
    let car = Car::new(String::from("Toyota"), 200, Box::new(dog));
    // 再卡车声浪
    car.play_sound();  // 输出：Toyota maked the 200's housepower car  和 Buddy says Woof!
}
