use std::vec;


// remark: when a vector gets dropped, so does its elements.
pub fn vector_syntax() {

    // creating vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);

    assert_eq!(v, vec![1,2]);

    // Accessing element in two ways. 
    let _m: i32 = v[1]; // will panic! at runtime if out-of-bound
    let n: Option<&i32> = v.get(1); // returns Option<&T> type
    match n {
        Some(x) => println!("2nd elt is {}", x),
        _ => println!("2nd elt does not exist"),
    }


    // iterate

    let mut v = vec![1,2,3];

    for i in &v { 
        println!("{}", i); 
    }

    for i in &mut v { 
        *i = (*i) * (*i); 
    }

    assert_eq!(v, vec![1,4,9]);
    
}


// note that strings can contain any UTF-8 char.
pub fn string_syntax() {
    
    // ways to create a string: 
    let s0: String = "str".to_string();

    let s1: String = String::from("str");

    let mut s2: String = String::new();
    s2.push_str("st");
    s2.push('r');

    let temp1 = String::from("st");
    let temp2 = String::from("r");
    let s3 = temp1 + &temp2; 
    // now temp1 is dropped and can no longer be used, but temp2 is still valid.

    assert_eq!(s0, s1);
    assert_eq!(s1, s2);
    assert_eq!(s2, s3);

    // string format
    let formated: String = format!("{s0}-{s1}");
    assert_eq!(formated, "str-str".to_string());


    // as explained in The Book, rust does not allow direct access via 
    // indexing; the following however is one way to access each char.

    let s = "this is a string".to_string();

    let mut c: std::str::Chars = s.chars();

    let n: Option<char> = c.nth(1);

    match n {
        Some(c) => println!("The second char is {}", c), 
        _ => println!("The second char does not exist"),
    }

}


use std::collections::HashMap;

pub fn hashmap_syntax() {

    // create and ownership transfer
    let mut price: HashMap<String, i32> = HashMap::new();

    let acorn_name = "acorn".to_string();
    let bread_name = "bread".to_string();

    // insert function transfers ownership of its arguments.
    price.insert(bread_name, 4);
    price.insert(acorn_name, 3);

    // acorn_name and bread_name ownerships are moved into the hashmap, 
    // and can no longer be used at this point.


    // element access

    fn get_price(name: &String, data: &HashMap<String, i32>) -> i32 {
        data.get(name) // returns Option<&T> type
            .copied()
            .unwrap_or(0)
    }

    println!("Bread price is {}", get_price(&"bread".to_string(), &price));


    // overwrite via inserting again

    price.insert("potato".to_string(), 10);
    assert_eq!(10, price.get(&"potato".to_string()).copied().unwrap_or(0));

    price.insert("potato".to_string(), 15);
    assert_eq!(15, price.get(&"potato".to_string()).copied().unwrap_or(0));


    // insert only if not present
    price.entry("potato".to_string()).or_insert(20);
    assert_eq!(15, price.get(&"potato".to_string()).copied().unwrap_or(0));


    // update existing value

    let mut freq: HashMap<i32, u32> = HashMap::new();

    let numbers: [i32; 10] = [-2, 8, 6, 1, 6, 6, 3, -2, 0, 5];

    for n in numbers {
        let count = freq.entry(n).or_insert(0);
        *count += 1;
    }


    // iterate; ordering seems arbitrary.

    for (key, value) in &freq {
        println!("{}: \t{}", key, value);
    }

}


