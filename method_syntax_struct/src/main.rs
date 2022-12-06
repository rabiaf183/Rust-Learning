#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{//&self to not take the ownership to change the struct &mut self

    // associated functions: don't have self as first parameter
    fn area(&self)->u32{
        self.width*self.height
    }
    //fn width(&self)-> bool{
    //    self.width > 0
    //}
    fn can_hold(&self , other: &Rectangle) ->bool{
        self.width > other.width && self.height > other.height
    }
}

/* 

fn main() {
    //methods define within the context of a struct, enum, object 

     // first parameter is always self...... Rectangle area
    let rect1= Rectangle{
        width: 10,
        height: 50,
    };
    if rect1.width(){
        println!(" the rectangle has non zero width {}", rect1.width());
    }
    println!(" the area of rectangle {} in pexel square ", rect1.area());
  
}*/
// we give methods with the same name as field so to retuen the value in the same field and do nothing else
// we have methods called getters for this. we can make the filed private but the method is public and thus only read only access for the public api
// in c we use . to call method on an object and -> if we are calling a method on a pointer.
// object ->method == object.method().
// rust doesn't have -> rather automatic referencing 




/////////////************** Methods with more parameters******************
fn main()
{
    let rect1= Rectangle {
        width: 30,
        height: 50,
    };
    let rect2= Rectangle {
        width: 10,
        height: 40,
    };
    let rect3= Rectangle {
        width: 60,
        height: 50,
    };
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}