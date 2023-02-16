//! This file is dedicated to record some gotchas in the Rust language, which 
//! may potentially cause nasty runtime bugs if one is not being careful. 


/// The statement `x.unwrap_or(<expr>)` will always eagerly evaluate `<expr>`, 
/// regardless of the actual value of `x`. When the evaluation of `<expr>` is 
/// effectful (ie. requires function call), unexpected behaviors may emerge.
mod effectful_unwrap_or {

#[allow(dead_code)]
  fn boom() {
    println!("Boom!");
    println!("Boom!");
    println!("Boom!");
  }

  #[test]
  fn effect() {
    let _ = Some(()).unwrap_or(boom()); // will explode
  }
  
}