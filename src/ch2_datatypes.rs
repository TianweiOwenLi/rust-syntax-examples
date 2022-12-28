
// Numeric types are intuitive. Operations include + - * / %.
pub fn numeric() {
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
pub fn tuple() {
    let tup: (i32, i32) = (100, 66);
    let (_x, y): (i32, i32) = tup;
    assert_eq!(y, 66); 
    assert_eq!(tup.0, 100);
}


// arrays are also intuitive
pub fn array() {
    let arr2: [i32; 6] = [233,233,233,233,233,233];
    let mut arr3: [i32; 6] = [233; 6];

    assert_eq!(arr2, arr3);
    assert_eq!(arr2[3], 233);

    arr3[0] = 666;
    assert_eq!(arr3[0], 666);   
}
