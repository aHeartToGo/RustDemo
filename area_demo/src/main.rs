//简化代码
use std::ops::Mul;
use std::convert::{Into};

// 为结构体添加两个泛型
struct Rect<T, U>{
    width: T,
    height: U
}

// 方法实现
impl<T, U> Rect<T, U> {
    fn area(&self) -> T
    //泛型约束
    where T: Mul<Output = T> + Copy,
          U: Into<T> + Copy {
        self.width.mul(self.height.into())
    }
}

fn main() {
    let rect = Rect{width:2.5, height:7};
    println!("rect width is {}", rect.width);
    println!("rect height is {}", rect.height);
    println!("rect area is {}", rect.area());
}
