const KEY : &'static str = "BEST";
const MOD_NUMBER : u8 = 26;
const KEY_MOD : u8 = 4;
fn main() {
    let x = "hello".to_ascii_uppercase();
    println!("result is {}",generate_encipher(&x))
    // println!("Hello, world! ascii a is {} ",x);
}
pub fn generate_encipher(plaintext : &str) -> String {
    let byte_string = plaintext.as_bytes().to_vec();
    println!("byte_string is {:?}",byte_string);
    let  result = byte_string.iter().inspect(|z|{
        dbg!(z);
    }).enumerate().map(|(index,el)| (el.to_le()  + (KEY[..=(index   % KEY_MOD as usize) ] ).as_bytes()[0])%MOD_NUMBER ).inspect(|z|{
        dbg!(z);
    }).collect::<Vec<u8>>();
    println!("{:?}",result);
    let resultx = result.iter().map(|el|el.to_le()+ 65 as u8).collect::<Vec<u8>>();
    println!("{:?}",resultx);
    String::from_utf8(resultx).unwrap()
}












