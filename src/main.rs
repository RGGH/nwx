use std::fs;
use std::path::Path;
use std::io::{stdin, Write};
use colored::Colorize; // Assuming you're using the `colored` crate for text coloring

fn main() {
    let net = Path::new("/sys/class/net");
    let entry = fs::read_dir(net).expect("Error");
    let ifaces = entry
        .filter_map(|p| p.ok())
        .map(|p| p.path().file_name().expect("Error").to_os_string())
        .filter_map(|s| s.into_string().ok())
        .collect::<Vec<String>>();

    println!("These are the available interfaces: {:?}", ifaces);

    let mut iface_path: Option<std::path::PathBuf> = None;

    while iface_path.is_none() {
        println!("\nChoose an interface: ");

        // Create a mutable String and store the user's input
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        // !! important - remove newline character from input
        let iface_choice = input.trim();
        // Construct the path based on the chosen interface
        let potential_iface_path = net.join(iface_choice).join("address");

        // Check if the potential path exists (i.e., if the interface choice is valid)
        if potential_iface_path.exists() {
            iface_path = Some(potential_iface_path);
        } else {
            println!("Invalid interface name. Please choose from the available interfaces.");
        }
    }

    if let Some(iface_path) = iface_path {
        println!("Chosen interface path: {:?}", iface_path);

        let mut f = fs::File::open(&iface_path).expect("Failed");
        let mut macaddr = String::new();
        f.read_to_string(&mut macaddr).expect("Error");
        println!("MAC address: {}", macaddr.trim().green());
    }
}

