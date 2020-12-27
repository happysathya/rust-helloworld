use chrono::Utc;

fn main() {
    println!("{}: {}", timestamp(), "Hello World!!!");
}

fn timestamp() -> String {
    Utc::now().to_string()
}