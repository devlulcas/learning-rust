use ferris_says::say; 

fn main() {
    let stdout = std::io::stdout();
    
    let message = String::from("Hello, world!");

    let mut writer = std::io::BufWriter::new(stdout.lock());

    say(
        message.as_bytes(),
        message.chars().count(),
        &mut writer,
    ).unwrap();
}
