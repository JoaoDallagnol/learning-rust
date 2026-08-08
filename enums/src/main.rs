fn main() {
    enum IpAddrKind {
        V4,
        V6
    }

    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {}

    route(ip_kind: IpAddrKind::V4);
    route(ip_kind: IpAddrKind::V6);

    // Using strucs
    struct IpAddr {
        kind: IpAddrKind,
        addres: String
    }

    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        addres: String::from("127.0.0.1");
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        addres: String::from("::1");
    };

    // ----------------
    // Using types
    enum IpAddrKindType {
        V4(String),
        V6(String)
    }

    let home = IpAddrKindType::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKindType::V6(String::from("::1"));

    //-----------
    // Using enhance enums
    enum IpAddrEnhance {
        v4(u8,u8,u8,u8),
        V6(String)
    }

    let homeEnhance = IpAddrEnhance::V4(127,0,0,1);
    let loopbackEnhance = IpAddrKindType::V6(String::from("::1"));

}
