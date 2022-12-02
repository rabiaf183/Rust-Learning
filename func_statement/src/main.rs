fn main() {
    let x = plus_one(5);
    //let x = (let y = 6); statements dont return values so we can't assign a let to inside statement
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {// -> rerturns last line implicitly
    x + 1
}

