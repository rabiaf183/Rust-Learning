/*fn main() {
    
        let mut s = String::from("hello");
    
        let len = calculate_length(&mut s);// value of s will not get dropped as it is not own by len
    
        println!("The length of '{}' is {}.", s, len);
        change(&mut s);
        println!("the string is{}",s);
        

    }
    
    fn calculate_length(s1: & String) -> usize { // s is a reference to a String I can read but can't modify if s is nt mutable
        s1.len()
    }// here s goes out of scope but does not drop as it doesn't have given the ownersip
    fn change(some_string: &mut String){
        some_string.push_str(",world");
        println!("the string is {}",some_string );
    }*/
    fn main() {
        let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("the value r1 {}", r1);// we could tacke this down by creating a different scope
    }
        let r2 = &mut s;
    
       // println!("{}, {}", r1, r2);// this is because the race condition where two or more pointers access the same data
    

    }
    
    

