/*fn main()
{
let x=5;                      // variable x: i32
let (_a,_b)= (1, 5);                // patterns
                                /* annotations*/
let _x: i32=5; // statically typed language. doesn't require to explicitly define the type of variabe
x is binding with the type i32 and the value is 5

                            Mutability
let _v=5; 
x=10; will give an error
let mut _x=5; 
_x=10;

                         /* Initializing Bindings*/

    //let x;// bindings must have a value to initalize with
    let x: i32= 5;
    
    println!("The value of x is {}", x);
    
}
*/   


              //SCOPE        
/*fn main() {
    let x: i32 = 17;
    //scope
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is ", x); // This won't work.
}*/


               // SHADOW

/*fn main() {
let x: i32 = 8;
{
    println!("{}", x); // Prints "8".
    let x = 12;
    println!("{}", x); // Prints "12".
}
println!("{}", x); // Prints "8".
let x =  42;
println!("{}", x); // Prints "42".
}*/


fn main() {
let mut x: i32 = 1;
x = 7;
let x = x; // `x` is now immutable and is bound to `7`.
println!(" value of x {}", x);

let y = 4;
let y = "I can also be bound to text!"; // `y` is now of a different type.
println!(" value of y {}:", y);


}



