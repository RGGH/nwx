use colored::Colorize;
use std::fs;
use std::io::{stdin, Read};
use std::path::Path;

fn main() {
    // Network Inteface Location (Ubuntu/Linux Mint)
    let net = Path::new("/sys/class/net");
    let entry = fs::read_dir(net).expect("Error");

    // build vector of intrerface names
    let ifaces = entry
        .filter_map(|p| p.ok())
        .map(|p| p.path().file_name().expect("Error").to_os_string())
        .filter_map(|s| s.into_string().ok())
        .collect::<Vec<String>>();

    println!("These are the available interfaces: {:?}", ifaces);
    println!("\nChoose an interface to view MAC address :\n");
    let mut input = String::new();
    // Store the user's input
    stdin().read_line(&mut input).expect("Failed to read line");

    // !! important - remove newline character from input
    let iface_choice = input.trim();

    // Construct the path based on the chosen interface in "/sys/class/net"
    let iface_path = net.join(iface_choice).join("address");

    // get the MAC address of chosen interface, print to screen and write to txt file
    get_mac(&iface_path);
    
}

fn get_mac(iface_path: &Path) -> String {
    println!("Chosen interface path: {:?}", iface_path);
    // read the contents of the 'device'
    let mut f = fs::File::open(iface_path).expect("Failed");
    let mut macaddr = String::new();
    // read MAC and print to screen
    f.read_to_string(&mut macaddr).expect("Error");
    println!("MAC address: {}", &macaddr.to_string().green());

    let _out = std::fs::write("mac_info.txt", &macaddr);
    macaddr
}
