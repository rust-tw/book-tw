fn main() {
    // ANCHOR: here
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("寫死的 IP 位址應該要有效");
    // ANCHOR_END: here
}
