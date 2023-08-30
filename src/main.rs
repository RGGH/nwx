use colored::Colorize;
use std::fs;
use std::io::{stdin, Read};
use std::path::Path;

fn main() {
    // Network Inteface Location (Ubuntu/Linux Mint)
    let net = Path::new("/sys/class/net");
    let entry = fs::read_dir(net).expect("Error");

    let ifaces = entry
        .filter_map(|p| p.ok())
        .map(|p| p.path().file_name().expect("Error").to_os_string())
        .filter_map(|s| s.into_string().ok())
        .collect::<Vec<String>>();

    println!("These are the available interfaces: {:?}", ifaces);
    println!("\nChoose an interface: ");
    // Create a mutable String and store the user's input
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    // !! important - remove newline character from input
    //let iface_choice = input;//.trim();
    let iface_choice = input.trim();
    // Construct the path based on the chosen interface
    let iface_path = net.join(iface_choice).join("address");

    // Now you can use `iface_path` to further work with the chosen interface's path
    println!("Chosen interface path: {:?}", iface_path);
    let mut f = fs::File::open(iface_path).expect("Failed");
    let mut macaddr = String::new();
    f.read_to_string(&mut macaddr).expect("Error");
    println!("MAC address: {}", macaddr.to_string().green());
}
