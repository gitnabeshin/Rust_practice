fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct RectangleD {
    width: u32,
    height: u32,
}

impl RectangleD {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &RectangleD) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> RectangleD {
        RectangleD {
            width: size,
            height: size,
        }
    }
}

impl RectangleD {
    fn square_double(size: u32) -> RectangleD {
        RectangleD {
            width: size * 2,
            height: size * 2,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 20;
    println!("1. The area of Rectangle: {}", area(width1, height1));
    println!("   print values: width: {}, height: {}", width1, height1);

    let dim = (30, 40);
    println!("2. The area of Rectangle: {}", area2(dim));
    println!("   print values: width: {}, height: {}", dim.0, dim.1);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("3. The area of Rectangle: {}", area3(&rect));
    println!(
        "   print values: width: {}, height: {}",
        rect.width, rect.height
    );

    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 20,
    // };
    // this will be error
    // the trait `std::fmt::Display` is not implemented for `Rectangle`
    //println!("print values of Rectangle: {:?}", rect2);

    let rect_d = RectangleD {
        width: 30,
        height: 40,
    };
    println!("4. The area of Rectangle: {}", rect_d.area());
    // #[derive(Debug)] macro allows following
    println!("   print values: {:?}", rect_d);
    println!("   print values: {:#?}", rect_d);

    let rect_1 = RectangleD {
        width: 30,
        height: 50,
    };
    let rect_2 = RectangleD {
        width: 10,
        height: 40,
    };
    let rect_3 = RectangleD {
        width: 60,
        height: 45,
    };
    println!("5. Struct function");
    println!("   rect_1: {:?}", rect_1);
    println!("   rect_2: {:?}", rect_2);
    println!("   rect_3: {:?}", rect_3);
    println!("   Can rect_1 hold rect_2? {}", rect_1.can_hold(&rect_2));
    println!("   Can rect_1 hold rect_2? {}", rect_1.can_hold(&rect_3));

    let square_1 = RectangleD::square(10);
    println!("6. Struct function");
    println!("   values: {:?}", square_1);
    println!("   area: {:?}", square_1.area());

    let square_2 = RectangleD::square_double(10);
    println!("7. Struct function");
    println!("   values: {:?}", square_2);
    println!("   area: {:?}", square_2.area());

}
