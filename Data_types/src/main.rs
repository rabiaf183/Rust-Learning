/*fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
     OR 
    let tup: (i32, f64, u8) = (500, 6.4, 1);


    println!("The value of y is: {y}");
    */
    //let guess: u32 = "42".parse().expect("Not a number!"); error 
    fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    
    let t = true;
    let f: bool = false; // with explicit type annotation
     
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!(" the value of x {}, y {}, c {}, z {} is", x,y,c,z);
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
 


}




