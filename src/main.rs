#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let deck = Deck { cards: vec![] };
    let dec1: Deck = Deck { cards: Vec::new() };
    // vec![] and Vec::new()are same
    println!("{:#?}", deck);
    println!("{:?}", dec1);
}
