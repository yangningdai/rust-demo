use rand::prelude::*;
use std::{net::Ipv4Addr, str::FromStr};
use roaring::RoaringBitmap;

fn main() {
    let mut ips: Vec<String> = Vec::new();
    let mut bmp = RoaringBitmap::new();
    // loop 1 to 100
    for _i in 1..21 {
        let ip = gen_ip();
        let i32 = ip_to_u32(&ip);
        bmp.insert(i32);
        ips.push(ip);
    }
    // loop bmp
    for i in bmp.iter() {
        let ip = u32_to_ip(i);
        //if ip in ips
        if ips.contains(&ip) {
            println!("ip: {}, in ips", ip);
        } else {
            println!("ip: {}, not in ips", ip);
        }
    }
    
}
//gen random ip string
fn gen_ip() -> String {
    let mut rng = rand::thread_rng();
    let mut ip = String::new();
    for _ in 0..4 {
        ip.push_str(&rng.gen_range(0..255).to_string());
        ip.push_str(".");
    }
    ip.pop();
    ip
}

fn ip_to_u32(ip: &str) -> u32 {
    let addr = Ipv4Addr::from_str(ip).unwrap();
    addr.into()
}

// u32 to ip string
fn u32_to_ip(u32: u32) -> String {
    let addr = Ipv4Addr::from(u32);
    addr.to_string()
}
