use rand::{rng, seq::SliceRandom};

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
                let card: String = format!("{value} of {suit}");
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck: Deck = Deck::new();

    deck.shuffle();
    let cards: Vec<String> = deck.deal(3);

    println!("Heres your deck: {:#?}", deck);
    println!("Heres your hand: {:#?}", cards);
}
