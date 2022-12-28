
pub fn struct_syntax() {

    struct Point {
        x_coord: f32,
        y_coord: f32,
    }

    let _q: Point = Point{x_coord: 0.0, y_coord: 0.0};

    let mut p: Point = Point{ // note that we can swap ordering.
        y_coord: 666.6, 
        x_coord: 233.3,
    };

    p.y_coord = 6666.6;

    fn build_point(x_coord: f32, y_coord: f32) -> Point {
        Point{x_coord, y_coord}
    }

    let mut r: Point = build_point(0 as f32, 0 as f32);
    r.x_coord = 2333.3;

}

pub fn special_struct() {

    // tuple structs without named fields; 
    // note the round brackets instead of curly.
    struct ColorRGB(u8, u8, u8);
    let _green = ColorRGB(0, 255, 0);


    // unit-like struct without any fields
    struct Unit;
    let _x: Unit = Unit;


    // printable struct
    #[derive(Debug)]
    struct Student {
        _name: String, 
        _id: u32,
    }
    println!("{:?}", Student{_name: String::from("bob"), _id: 666});        

}

// struct lifetimes are skipped till later.

pub fn struct_method() {

    struct Rectangle{
        width: u32, 
        height: u32, 
    }

    // implementations of struct methods can span across several 'impl' blocks.
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn transpose(&self) -> Self {
            Self{ // 'Self' refers to type of self, in this case 'Rectangle'.
                width: self.height, 
                height: self.width,
            }
        }
    }

    let r = Rectangle{width: 5, height: 3};
    assert_eq!(r.area(), r.transpose().area());
}
