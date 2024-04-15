use hand_isomorphism_rust::deck::Card;


const BASE: u64 = 53; // Max card + 2

pub fn encode_cards(cards: &Vec<Card>) -> u64 {
    let mut encoded_cards: u64 = 0;
    for &card in cards {
        encoded_cards = encoded_cards * BASE + (card + 1) as u64;
    }

    return encoded_cards;
}

pub fn decode_cards(encoded_cards: u64) -> Vec<Card> {
    let mut cards: Vec<u8> = vec![];
    let mut encoded_value = encoded_cards;
    while encoded_value > 0 {
        cards.push((encoded_value % BASE) as Card);
        encoded_value /= BASE;
    }
    cards.reverse();
    return cards;
}
