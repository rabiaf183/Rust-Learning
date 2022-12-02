fn main() {
   
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    

    println!("{}", s); // This will print `hello, world!` rust call the function drop to stop s out of scope

    let s1 = String::from("hello");
    let s2 = s1.clone();//if the string copies from s1 to s2 then the heap gets copies and if we delete the memory for s1 then s2 
    //will give us double memory deletion bug. so instead of shallow copying rust tranfer the heap data to s2 but even the if we want both s1 and s2 to exist use clone. 

    println!("{},{}", s1, s2);
   
        let x = 5;// this is valid but contradictory to what we have seen for strings 
        // because integers have a fixed size which get directly store on the stack instead of heap
        // so there is no need to make invalid x hence no difference between shallow and deep copy. so no need to clone with deep copy
        let y = x;
    
        println!("x = {}, y = {}", x, y);
    
    
}









