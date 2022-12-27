

fn variable() {

    // Variable declaration. Type-annotation is optional.
    let x: i32 = 5;


    // 'x = x + 3' will cause an error because variables are by default 
    // immutable. We can create a new variable named 'x' to shadow the old one.
    let x: i32 = x + 3;
    assert_eq!(x, 8); // '!' stands for a macro.


    // mutable variable can be changed
    let mut m: i32 = 5;
    m += 3;
    assert_eq!(m, 8);


    // constants are calculated at compile time; type annotation is mandatory.
    const _CONSTANT_VAR: i32 = 666 * 233;
}


// variables have their own (explicit) scope.
fn scope() {
    let v: i32 = 233;
    {
        let v: i32 = 666;
        assert_eq!(v, 666);
    }
    assert_eq!(v, 233);
}


// Numeric types are intuitive. Operations include + - * / %.
fn numeric() {
    let _b: bool = true;
    let _x: i32 = 233;
    let _y: u64 = 666666;
    let _z: f32 = 0.0;
    let _hex: i64 = 0xdeadbeef;
    let _oct: i64 = 0o233;
    let _bin: i8 = 0b0011010;
    let _byte: u8 = b'?'; // type must be 'u8' and only for ascii chars.
}


// tuples work similar to that of python.
fn tuple() {
    let tup: (i32, i32) = (100, 66);
    let (_x, y): (i32, i32) = tup;
    assert_eq!(y, 66); 
    assert_eq!(tup.0, 100);
}


// arrays are also intuitive
fn array() {
    let arr2: [i32; 6] = [233,233,233,233,233,233];
    let mut arr3: [i32; 6] = [233; 6];

    assert_eq!(arr2, arr3);
    assert_eq!(arr2[3], 233);

    arr3[0] = 666;
    assert_eq!(arr3[0], 666);   
}


fn function_call() {
    // Return type annotation is mandatory except when nothing is returned
    fn plus_one(num: i32) -> i32 {
        num + 1
    }

    let mut x: i32 = 0;
    x = plus_one(x);
    assert_eq!(x, 1);


    // note that rust implements auto referencing / dereferencing for fn calls.
    let s: String = String::from("bob");
    let _l: usize = s.len(); // valid
    let _l: usize = (&s).len(); // also valid
}


// 'if' works like usual except that it can be treated as a value
fn if_syntax() {
    // as a procedure
    if true {
        // do something
    } else {
        // do something else
    }

    // as a value / ternary operator
    let x: i32 = if true {3} else {4};
    assert_eq!(x, 3);
}


// 'loop' never ends unless 'break' is reached. It's also a value.
fn loop_syntax() {

    let mut x: i32 = 3;

    let y: i32 = loop {

        // can optionally follow 'break' with a return value
        if x >= 10 {break x * 2;}

        x += 1;
    };

    assert_eq!(y, 20);
}


// while and for loops are intuitive.
fn for_and_while() {
    
    let mut x: i32 = 10;

    while x > 0 {
        x -= 1;
    }
    assert_eq!(x, 0);

    let mut a: [i32; 3] = [233, 666, 999];
    for element in a {
        x += element;
    }
    assert_eq!(x, 233 + 666 + 999);

    for index in 0..3 { // syntax: 'inclusive..exclusive'.
        a[index] = index as i32; // explicitly cast from usize to i32
    }
    assert_eq!(a, [0,1,2]);
}


// I suggest reading this chaper. This is more conceptual than syntactic.
// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html


// Rust's Ownership Model: 
//  * Each value in Rust has an owner
//  * There can only be one owner at a time
//  * When an owner goes out of scope, the value is dropped (becomes invalid)


// for things on heap, rust has shallow transfer instead of shallow copy.
fn shallow_transfer() {

    // rust's "shallow copy" of object is actually "move"
    let _a: String = String::from("bob");

    let _b: String = _a;  
    // at this point _a becomes invalid, because the ownership of the string is 
    // transferred to _b during the assignment, instead of having both pointers 
    // own the same heap memory. Eliminates a whole class of bug. Hooray!

}


// rust's deep copy is intuitive
fn deep_copy() {

    let _b: String = String::from("bob");
    let _c: String = _b.clone(); 
    // after this point we can still use _b, because the heap memory they point
    // to is duplicated.

}


// copy works straightforward for things on stack though
fn stack_copy() {
    let m: i32 = 50;
    let n: i32 = m;
    // we can still use both m and n, because they are on stack, and is thus 
    // copied over.
    assert_eq!(m, n); 
}


fn ownership() {

    // rust automatically drops values that are out of scope.
    {
        let _s: String = String::from("bob");
    } // here, '_s' is dropped


    let s1: String = String::from("bob"); // s1 enters scope
    takes_ownership(s1); 

    // s1 moves into the function...and gets "dropped" when function call ends.
    // Thus after the takes_ownership() function call, we can no longer use s1.

    
    let n1: i32 = 666; // n1 enters scope
    copy_by_value(n1);
    let _n2: i32 = n1 + 3;

    // n1 gets copied into the function, so even if n1's copy is dropped by the 
    // end of function call, n1 itself is unaffected, and still can be used.


    // helper functions below
    fn takes_ownership(s: String) {
        println!("{s}");
    }

    fn copy_by_value(n: i32) {
        println!("{n}");
    }
}


// creating a reference  to a valueis called "borrowing": we make use of the  
// value without taking its ownership. 
fn immutable_references() {

    let s: String = String::from("bob"); // create value

    let ref_of_s: &String = &s; // create immutable reference

    let _len: usize = calculate_length(ref_of_s); // use immutable reference

    // Can still use 's' because by creating 'ref_of_s', we have created a 
    // reference to s without owning it.
    println!("{}", s);

    // We can even use such a reference again, the reason being that references 
    // themselves are copied by value.
    calculate_length(ref_of_s);


    let _len: usize = calculate_length_and_take_ownerthip(s);
    // can no longer use s at ths point.


    // borrowing
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    // taking ownership
    fn calculate_length_and_take_ownerthip(s: String) -> usize {
        s.len()
    }
}


// Borrowing has an issue: you cannot modify borrowed stuff via its reference. 
// Thus we will need mutable references instead.
fn mutable_references() {

    // must declare 'bob_name' as mutable in order to borrow it as mutable
    let mut bob_name: String = String::from("bob");

    // borrow 'bob_name' as mutable via creating a mutable reference to it.
    let ref_of_name: &mut String = &mut bob_name;

     // we can only modify mutable references, but not regular ones
    change(ref_of_name);


    fn change(s: &mut String) { // takes in a mutable reference as argument
        s.push_str(" likes alice");
    }
}

// Mutable references has a restriction: when there's a mutable reference to a 
// value, no other references (mutable or not) to such value are allowed. 
// 
// This makes sense: immutable references can be thought of as having permission
// for reading, and mutable references can be thought of as having permission 
// for both reading and writing. To prevent a race condition, a piece of data 
// can have multiple readers; but when a writer exists, nothing else should 
// access the data. 
// 
// Summary: we can one of the following to a value:
//  * no reference, 
//  * arbitrarily many immutable reference (ie. 'read mode'), or
//  * only one mutable reference (ie. 'write mode').
//


// What if we have created some immutable refs, we are done using them, and now 
// we want mutable refs? How can we get rid of the old immutable refs? They 
// cannot coexist, can they?
// 
// For now, we can interpret them as an automatic process: once we create a 
// mutable ref, all previous immutable refs are automatically dropped; and once 
// we start creating immutable refs, the existing mutable ref (if there is one) 
// is automatically dropped. 
//
// One tip from author: manage the references as if you were managing 
// read / write concurrency of a program! For readers who are familiar with 
// C / C++, it would be nice to make an analogy to shared mutex.
//
fn switch_between_references() {
    let mut bob_name: String = String::from("bob");

    // arbitrarily many immutable references to value are allowed, as long as 
    // there are no mutable references hanging around.
    let immut_1: &String = &bob_name; // create immutable ref
    let immut_2: &String = &bob_name; // create immutable ref

    take_ownership(immut_1); // use immutable ref
    take_ownership(immut_2); // use immutable ref

    // immutable references are not used after this point, 
    // so now we can proceed to create mutable references.

    // Create mutable ref. 'immut_1' and 'immut_2' gets dropped at this point.
    let mutab_1: &mut String = &mut bob_name;
    // note: we cannot create a second mutable ref.

    change(mutab_1); // use mutable ref

    // Create immutable ref. 'mutab_1' gets dropped at this point
    let ref_of_name_3: &String = &bob_name;

    take_ownership(ref_of_name_3); // use immutable ref


    // helper functions
    fn take_ownership(s: &String) -> usize {
        s.len()
    }

    fn change(s: &mut String) {
        s.push_str(" likes alice");
    }

}


fn slice() {
    
    // slices work similar to python slicing.
    let s: &str = "hello rust lang";
    let _hello: &str = &s[..5];
    let _rust: &str = &s[6..10];
    let _lang: &str = &s[11..];

    let t: String = String::from("hello rust lang");
    let _rust: &str = &t[6..10];

    // integer slices work in a similar fashion
    let a: [i32; 5] = [1,2,3,4,5];
    let x: &[i32] = &a[1..4];
    assert_eq!(x, &[2,3,4]);

}


fn slice_ownership() {

    let mut s: String = String::from("bob likes alice"); // create mutable var

    let _word: &str= first_word(&s); // create immutable ref

    s.clear(); // mutable borrow occurs, previous immutable ref becomes invalid

    // can no longer use '_word' here.

    // remark: rust auto cleans up references to invalid (ie. freed) data.


    // helper function
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {return &s[..i];}
        }

        &s[..]
    }
}


fn struct_syntax() {

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

fn special_struct() {

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

fn struct_method() {

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


fn enum_syntax() {

    // enum can take arguments
    enum Money {
        _Dime, 
        Quarter, 
        CustomCheck(u32),
    }

    let _money_1 = Money::Quarter;
    let _money_2 = Money::CustomCheck(100);


    // rust has builtin Option type, implemented as enum
    let _x: Option<i32> = Some(666);
    let _y: Option<i32> = None;

}


use std::cmp::max;


// in general, one can use 'match' to deconstruct enum.
fn match_syntax() {

    let x: Option<i32> = Some(666);
    let y: Option<i32> = None;

    fn opt_max(x: Option<i32>, y: Option<i32>) -> Option<i32> {
        match (x, y) {
            (Some(a), Some(b)) => Some(max(a, b)), 
            (_, None) => x, 
            _ => y,
        }
    }

    assert_eq!(x, opt_max(Some(233), opt_max(x, y)));
    assert_eq!(None, opt_max(None, None));
}


// 'if let' syntax is an alternative to 'match'
fn iflet_syntax() {

    let x: Option<i32> = Some(666);
    let y: Option<i32> = None;

    fn opt_max(x: Option<i32>, y: Option<i32>) -> Option<i32> {
        if let (Some(a), Some(b)) = (x, y) {
            Some(max(a, b))
        } else if let (_, None) = (x, y) {
            x
        } else {
            y
        }
    }

    assert_eq!(x, opt_max(Some(233), opt_max(x, y)));
    assert_eq!(None, opt_max(None, None));
}

fn main() {
    // basic chapter
    variable();
    scope();

    // datatype
    numeric();
    tuple();
    array();

    // control flow
    function_call();
    if_syntax();
    loop_syntax();
    for_and_while();

    // copy chapter
    shallow_transfer();
    deep_copy();
    stack_copy();

    // ownership chapter
    ownership();
    immutable_references();
    mutable_references();
    switch_between_references();
    slice();
    slice_ownership();

    // struct
    struct_syntax();
    special_struct();
    struct_method();

    // enum pattern match
    enum_syntax();
    match_syntax();
    iflet_syntax();

}
