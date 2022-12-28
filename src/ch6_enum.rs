
pub fn enum_syntax() {

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
pub fn match_syntax() {

    let x: Option<i32> = Some(666); // Option<i32> is like 'int option' in ml.
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
pub fn iflet_syntax() {

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
