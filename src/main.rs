
mod encode;

use std::fs::File;
use std::io::BufWriter;
use bincode;

use encode::encode_cards;
use hand_isomorphism_rust::hand_indexer::HandIndexer;

static BATCH_SIZE: usize = 50_000_000;
static EXPORT_PATH: &str = "./exports";

fn save_hands_to_file(hands: &Vec<u64>, round: usize, batch: usize) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = format!("{}/round_{}_batch_{}.bin", EXPORT_PATH, round, batch);

    let file = File::create(filepath)?;
    let writer = BufWriter::new(file);
    bincode::serialize_into(writer, hands)?;
    Ok(())
}

fn generate_canonical_hands_and_save_to_files(indexer: &HandIndexer, round: usize) {
    let size = indexer.hand_indexer_size(round).unwrap() as usize;
    let mut cards = vec![0u8; 7]; // Adjust the size according to your needs
    let mut total_cards = 0u8;

    for i in 0..=round {
        total_cards += indexer.cards_per_round[i as usize];
    }

    let mut canonical_hands: Vec<u64> = vec![];
    let mut current_batch: usize = 0;

    // Generate canonical hands
    for i in 0..size {
        indexer.hand_unindex(round, i as u64, &mut cards);
        let hand = cards[..total_cards as usize].to_vec();
        let cards_id = encode_cards(&hand);
        canonical_hands.push(cards_id);

        if i > 0 && i % BATCH_SIZE == 0 {
            println!("Saving file for batch #{}", current_batch);
            save_hands_to_file(&canonical_hands, round, current_batch).expect("Failed to save data D:");

            current_batch += 1;
            canonical_hands.clear();
        }
    }

    if !canonical_hands.is_empty() {
        println!("Saving file for batch #FINAL");
        save_hands_to_file(&canonical_hands, round, current_batch).expect("Failed to save data D:");
    }
}

fn main() {
    // River indexer
    let hand_indexer = HandIndexer::new(4, &[2, 3, 1, 1]).unwrap();

    generate_canonical_hands_and_save_to_files(&hand_indexer, 3);
}
