use project_7b::ip; // import the ip module from lib.rs

fn main() {
    let ipv4 = ip::IP::new_v4("192.168.0.1");
    println!("{:?}", ipv4.read_ip());
}
