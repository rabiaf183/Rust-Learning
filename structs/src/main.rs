/*                      Structs Basics
fn main() {
    let mut user1= User{
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };
    let user2= User{
        active: user1.active,
        username: user1.username,
        email: String::from("anotherecample@"),
        sign_in_count: user1.sign_in_count,
        };
    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    // entire instance must be mutable 
    fn build_user(email: String, username: String)-> User{
       User{ email,
        username,
        active: true,
        sign_in_count: 1,
       }
    }
    user1.email=String::from("anotherexample@.com");
    //println!("print the user={}", user1);
    let black= Color(0,0,0);
    let origin=Point(0,0,0);
}
// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);*/
                    // Area of Rectangle: Rfeactoring with Tuples
    
fn main()
{
   // let width1= 30;
    //let height1= 50;
    let rect1= (30,50);
    println!("The area of rectangle is {} square pixels", area(rect1)
    );}

    fn area(dimension: (u32, u32))-> u32{
        dimension.0 * dimension.1
    }


