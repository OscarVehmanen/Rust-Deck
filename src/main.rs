#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = ["Ace", "Two", "Three"];
    
        let mut cards: Vec<String> = vec![];
    
        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }
    
        Deck { cards }
    }
}

fn main() {
    let deck = Deck::new();
    
    println!("Heres your deck: {:#?}", deck);
}
