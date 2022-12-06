
//#[derive(Debug)]
/*enum Usstate{
    Albama,
    Alaska,
}
enum Coin{
    Penny,
    Nickel,
    Quarter(Usstate),
}
fn main() {
    // The match control flow construct
    value_in_cents((Coin::Quarter(Usstate::Alaska)));
    
}
fn value_in_cents(coin: Coin)-> u8{
    match coin{
        Coin::Penny=>{
            println!("lucky penny");
            1
        }
        Coin::Nickel=> 5,
        Coin::Quarter(state)=> {
            println!("state quarter from{:?}", state);
            25
        }
    }
}
*/

// ADD One if we have some value
fn plus_one(x: Option<i32>)-> Option<i32>
{
    match x{
        None=> None,
        Some(i)=> Some(i+1), 
    }
}
fn main()
{
    let five= Some(5);
let six= plus_one(five);
let none= plus_one(None);
println!{"the value of six{:?}", six};
}
// THe placeholder_ in the catch all paterns // Exhaustiveness to cover all possible values
let dice_roll=9;
match dice_roll{
    3=> add_fancy_hat,
    7=> remove_fancy_hat,
    other=> move_space(other),
    // _ => reroll, if we don't want to perform any operation in the other match all values
    _ => (),
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll(){}
