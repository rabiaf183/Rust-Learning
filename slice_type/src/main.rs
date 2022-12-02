fn first_word(s: &String)-> &str
{
    let bytes= s.as_bytes();// as bytes convert the array to string of bytes. we do this to detect space
    for (i, &item) in bytes.iter().enumerate()// iter returns each element in a collection and enumerate wraps the result of iter and returns each element as a part of tuple
    {// the first element returned from enumerate is index and second is reference to the element
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
fn main()
{
    let mut s= String::from("hello world");
    
    let word =first_word(&s);// word will get the value 5
    println!("the value of s is {}", s);
    
    
    //s.clear();
    println!(" the value of word is {}", word);
    println!("the value of s is {}", s);
    // word is now totally invalid
    // word isnt connected to state of s and still contains 5
    // we could use word after calling s.clear
}
                  // SLICING
/*let s = String::from("hello");
let slice= &s[0..2];
// equal to above let slice=&s[..2];
let len=s.len();
let slice= &s[3..len];
// the slice of entire string let slice=&s[0..len] or [..]*/


             //Dangling Pointer
/*fn main() {
    let string = no_dangle();
    println!("string{}", string);
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}*/


