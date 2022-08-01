// mod lib;
// mod system_components;

// use system_components::{System};

pub fn main() {
    
    //let system = System::new();
    
    let op_code = 0xA0E0 as u16;

    let op_code_str = format!("{:04X}", op_code);
    let op = op_code_str.as_str();

    println!("{}",op_code_str);

    match op_code {
        0x00E0 => println!("Clear the display"),
        x if (x >= 0xA000 && x< 0xB000) => println!("MEM"),
        _ => println!("Unknown"),
    }

    println!("Hello, world!");
}
