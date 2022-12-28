
// I suggest reading this chaper from The Book. 
// This is more conceptual than syntactic.
// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html


// Rust's Ownership Model: 
//  * Each value in Rust has an owner
//  * There can only be one owner at a time
//  * When an owner goes out of scope, the value is dropped (becomes invalid)


// for things on heap, rust has shallow transfer instead of shallow copy.
pub fn shallow_transfer() {

    // rust's "shallow copy" of object is actually "move"
    let _a: String = String::from("bob");

    let _b: String = _a;  
    // at this point _a becomes invalid, because the ownership of the string is 
    // transferred to _b during the assignment, instead of having both pointers 
    // own the same heap memory. Eliminates a whole class of bug. Hooray!

}


// rust's deep copy is intuitive
pub fn deep_copy() {

    let _b: String = String::from("bob");
    let _c: String = _b.clone(); 
    // after this point we can still use _b, because the heap memory they point
    // to is duplicated.

}


// copy works straightforward for things on stack though
pub fn stack_copy() {
    let m: i32 = 50;
    let n: i32 = m;
    // we can still use both m and n, because they are on stack, and is thus 
    // copied over.
    assert_eq!(m, n); 
}


pub fn ownership_through_function() {

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
pub fn immutable_references() {

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
pub fn mutable_references() {

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
pub fn switch_between_references() {
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


pub fn slice() {
    
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


pub fn slice_ownership() {

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
