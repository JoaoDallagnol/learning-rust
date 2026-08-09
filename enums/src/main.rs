fn main() {
    enum IpAddrKind {
        V4,
        V6
    }

    let _four: IpAddrKind = IpAddrKind::V4;
    let _six: IpAddrKind = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Using strucs
    struct IpAddr {
        kind: IpAddrKind,
        addres: String
    }

    let _home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        addres: String::from("127.0.0.1")
    };

    let _loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        addres: String::from("::1")
    };

    // ----------------
    // Using types
    enum IpAddrKindType {
        V4(String),
        V6(String)
    }

    let _home = IpAddrKindType::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKindType::V6(String::from("::1"));

    //-----------
    // Using enhance enums
    enum IpAddrEnhance {
        V4(u8,u8,u8,u8),
        V6(String)
    }

    let _home_enhance = IpAddrEnhance::V4(127,0,0,1);
    let _loopback_enhance = IpAddrKindType::V6(String::from("::1"));

}
