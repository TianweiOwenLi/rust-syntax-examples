
pub fn function_call() {
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
pub fn if_syntax() {
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
pub fn loop_syntax() {

    let mut x: i32 = 3;

    let y: i32 = loop {

        // can optionally follow 'break' with a return value
        if x >= 10 {break x * 2;}

        x += 1;
    };

    assert_eq!(y, 20);
}


// while and for loops are intuitive.
pub fn for_and_while() {
    
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
