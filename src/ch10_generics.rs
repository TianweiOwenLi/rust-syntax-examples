
pub fn generic_type() {
    // we can implement a function of a generic type to reduce code duplication.
    fn _first_elem<T>(list: &[T]) -> Option<&T> {
        list.get(0)
    }

    // we can do the same thing for structs.
    struct Point<S, T> {
        _x: S,
        _y: T,
    }

    let _p: Point<i32, f32> = Point{_x: 3, _y: 5.0};

    // same for impl
    impl<T, S> Point<T, S> {
        fn _get_x(&self) -> &T {
            &self._x
        }
    }

    // impl for a specific type
    impl Point<i32, i32> {
        fn _create_default(&self) -> Self {
            Point{_x: 0, _y: 0}
        }
    }

    // same for Enums
    enum _MyOption<T> {
        Some(T),
        None,
    }
}


use std::cmp::Ordering;

// Note that sometimes  we will want the generic type T to have certain 
// features, such as being able to be compared when we make a function 
// `fn largest(list: &[T])`. This is called a 'trait' in rust. 
// In other languages, this may be called 'interface'. This is somewhat 
// comparable to signature in ML.
pub fn trait_for_generic() {

    struct Point<T: PartialOrd + PartialEq> {
        x: T,
        y: T,
    }


    // implement existing generics traits

    // impt<T: xxx> means "implement this only for type T that subscribes 
    // to trait xxx".
    impl<T: PartialOrd> PartialEq for Point<T> {
        fn eq(&self, other: &Self) -> bool {
            other.x == self.x && other.y == self.y
        }
    }

    impl<T: PartialOrd> PartialOrd for Point<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self.x > other.x { Some(Ordering::Greater) } 
            else if self.x < other.x{ Some(Ordering::Less) } 
            else { self.y.partial_cmp(&other.y) }
        }
    }

    let p1 = Point{x: 3, y: 5};
    let p2 = Point{x: 4, y: 4};

    assert!(p1 <= p2);

    
    // make and implement new traits; this can also be made generic.
    struct Circle {
        _center_y: f32,
        _center_x: f32,
        radius: f32,
    }

    struct Square {
        _center_x: f32,
        _center_y: f32, 
        side_len: f32,
    }

    trait Area {
        fn area(&self) -> f32;
    }

    impl Area for Circle {
        fn area(&self) -> f32 {
            self.radius * self.radius * 3.14
        }
    }

    impl Area for Square {
        fn area(&self) -> f32 {
            self.side_len * self.side_len
        }
    }
    
}


pub fn lifetime() {
    
}