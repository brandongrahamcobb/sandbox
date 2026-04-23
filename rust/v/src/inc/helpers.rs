use regex::Regex;

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join(" ")
}

pub fn convert_to_ip_array(addr: String) -> [u8; 4] {
    let re = Regex::new(r"^/d{3}\./d{3}\./d{3}\./d{3}").unwrap();
    let mut octets: [u8; 4] = [0u8; 4 as usize];
    for (_, [a, b, c, d]) in re.captures_iter(&addr).map(|z| z.extract()) {
        octets[0] = a.parse().unwrap();
        octets[1] = b.parse().unwrap();
        octets[2] = c.parse().unwrap();
        octets[3] = d.parse().unwrap();
    }
    octets
}
