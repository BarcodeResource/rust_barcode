extern crate connectcode;

use std::env;

use crate::connectcode::barcode::code39::Code39;
use crate::connectcode::barcode::industrial2of5::Industrial2of5;
use crate::connectcode::barcode::postnet::POSTNET;
use crate::connectcode::barcode::Barcode;

//cargo build --target wasm32-wasi

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 4 {
        eprintln!("{} <barcode> <input> <checkdigit>", program);
        return;
    }

    let barcode_type = args[1].clone();
    match barcode_type.as_ref() {
        "code39" =>
        {
            let input = args[2].clone();
            let check_digit:i32 = args[3].parse().unwrap();
            
            let mut code39  = Code39::new(&input,check_digit);
            println!("{}", code39.encode());        
            //println!("{}", code39.get_human_text());                    
            
        }
        "industrial2of5" =>
        {
            let input = args[2].clone();
            let check_digit:i32 = args[3].parse().unwrap();
            
            let mut industrial2of5  = Industrial2of5::new(&input,check_digit);
            println!("{}", industrial2of5.encode());        
            //println!("{}", industrial2of5.get_human_text());                    
            
        }
        "postnet" =>
        {
            let input = args[2].clone();
            
            let mut postnet  = POSTNET::new(&input);
            println!("{}", postnet.encode());        
            //println!("{}", postnet.get_human_text());                    
            
        }
        _ => eprintln!("{} is an unknown barcode type.", barcode_type)
        
        
    }

    
}