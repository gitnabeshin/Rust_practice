#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(iptype: IpAddrKind) {
    println!("route {:?}", iptype);
}

fn enum_basic(){
    let some_num = Some(5);
    let some_string = Some("a String");
    println!("{:?}", some_num);
    println!("{:?}", some_string);

    let absent_num: Option<i32> = None;
    println!("{:?}", absent_num);
}

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    route(home.kind);
    route(loopback.kind);
    println!("{}", home.address);

    enum_basic();

}

