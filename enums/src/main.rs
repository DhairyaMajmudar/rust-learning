// enums

#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

struct IPAddresses {
    address: String,
    kind: IPAddrKind,
}
fn main() {
    let ip_addr1 = IPAddresses {
        address: String::from("1.0.1"),
        kind: IPAddrKind::V4,
    };

    route(ip_addr1);
}

fn route(ip: IPAddresses) {
    print!("ip address: {} of kind {:?}", ip.address, ip.kind);
    // dbg!("{}", kind);
}
