
pub fn variable() {

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
pub fn scope() {
    let v: i32 = 233;
    {
        let v: i32 = 666;
        assert_eq!(v, 666);
    }
    assert_eq!(v, 233);
}
