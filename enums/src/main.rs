fn main() {
    
}
enum IpAddrKind{
    v4,
    v6,
}
struct IpAddr{
    Kind: IpAddrKind,
    address: String,
}
let home= IpAddr{
    Kind: IpAddrKind::v4,
    address: String::from("127.0.0.1"),
};
let loopback= IpAddr{
    Kind: IpAddrKind::v6,
    address: String::from("::1").
};
// OR
enum IpAddr{
    v4(String),
    v6(String),
}
let home= IpAddr::v4(String::from("127.0.0.1"));
let loopback= IpAddr::v6(String::from("::1"));
enum IpAddr{
    v4(u8, u8, u8, u8),
    v6(String),
}
let home= IpAddr::v4(127, 0, 0, 1);
let loopback= IpAddr::v6(String::from("::1"));

