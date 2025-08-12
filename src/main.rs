#[derive(Debug)] // make a println works
// * to use internal module you need to use mod prefix exmaples "mod" xxx;
// * for external crates we can access it directly exmaples "use" rand;

struct Deck {
    cards: Vec<String>,
}

// inherent implement function: should be same name as the struct
impl Deck {
    fn init() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        // expected the variable to be mutable
        let mut cards: Vec<String> = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        /* old way
        let deck: Deck = Deck { cards };
        return deck;
        */
        /* short hand return remember not to put ";" in the end
        Deck { cards }
        */
        /* shortest hand remember not to put ";" in the end and Self is the same as the struct name
        Self { cards }
        */
        Self { cards }
    }
    fn shuffle(&self) {}
}

fn main() {
    let deck: Deck = Deck::init();
    println!("cards in deck:{:#?}", deck.cards);
}
