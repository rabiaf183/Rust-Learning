// IMMUTABLE SHARED REFERENCES WITH SHARED MUTABILITY
/*fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("the value of {}, {}", r1 , r2); // solution to avoid immutable shared reference
    let r3 = &mut s; // BIG PROBLEM
    //println!("{}, {}", r1, r2);
    println!("{}", r3);// there can't be an exclusive reference untill we have shared reference of immutable reference
}*/

{fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}


}
