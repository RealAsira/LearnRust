// create a public enum "IP"
#[derive(Debug)]
pub enum IP {
    V4(String),
    V6(String),
}

// methods
impl IP {
  // make this method public, returns the IP as a string
  pub fn read_ip(&self) -> &str {
    match self {
      IP::V4(ip) => ip,
      IP::V6(ip) => ip,
    }
  }
}

// associated functions
impl IP {
  // make associated functions public, creates new IP instances
    pub fn new_v4(ipv4: &str) -> IP {
        Self::V4(ipv4.to_string())
    }

    pub fn new_v6(ipv6: &str) -> IP {
      Self::V6(ipv6.to_string())
    }
}
