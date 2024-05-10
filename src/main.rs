use xpath::xpath::XPath;

fn main() {
    let xpath_string = "/location[name=home]/device[name=macbook,os=macos]/memory";
    match XPath::parse(xpath_string) {
        Ok(xpath) => {
            println!("Parsed XPath object: {:?}", xpath);
            println!("String representation: {}", xpath);
        }
        Err(e) => println!("Error parsing XPath: {:?}", e),
    }
}
