mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let hex = &args[args.len() - 1];
        let tx = parser::decode_from_hex(&hex);
        println!("{:#?}", tx)
    } else {
        println!("Please provide a Bitcoin transaction in Hex");
    }
}
