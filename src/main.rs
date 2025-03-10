#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = ["Ace", "Two", "Three"];

    let mut cards: Vec<String> = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{value} of {suit}");
            cards.push(card);
        }
    }

    let deck: Deck = Deck { cards };
    println!("Heres your deck: {:#?}", deck);
}
