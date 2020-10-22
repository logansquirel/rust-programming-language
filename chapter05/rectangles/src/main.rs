fn main() {
    {
        let rect = Rectangle::new(30, 50);
        println!("rect = {:?}", rect);
        println!("rect = {:#?}", rect);
        println!("area = {}", rect.area());
    }
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        println!(
            "rect1 = {:?}\nrect2 = {:?}\nrect3 = {:?}",
            rect1, rect2, rect3
        );
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
    {
        let square = Rectangle::square(30);
        println!("square = {:?}", square);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
