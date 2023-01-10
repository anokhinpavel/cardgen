use clap::Parser;
use rand::Rng;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
struct CardGeneratorData {
    card_length: i32,
    bin: i32,
    number_of_cards: i32,
    path: std::path::PathBuf,
}

fn main() {
    let card_generator_data = CardGeneratorData::parse();
    let cards: Vec<String> = generate_cards(&card_generator_data);
    let mut file =
        File::create(card_generator_data.path).expect("Error encountered while creating file!");
    for card in cards {
        write!(file, "{}\n", card).expect("Write file error");
    }
}

fn generate_cards(card_generator_data: &CardGeneratorData) -> Vec<String> {
    let mut cards: Vec<String> = Vec::new();
    while cards.len() != card_generator_data.number_of_cards as usize {
        let mut card_number: String = card_generator_data.bin.to_string();
        while card_number.trim().len() != card_generator_data.card_length as usize {
            let num: u32 = rand::thread_rng().gen_range(0..9);
            card_number = format!("{}{}", card_number, num);
        }
        if luhn_validation(&card_number) {
            if !cards.contains(&card_number) {
                cards.push(card_number);
            }
        } else {
            continue;
        }
    }
    cards
}

fn luhn_validation(card_number: &String) -> bool {
    let chars: Vec<char> = card_number.chars().collect();
    let mut card_digits: Vec<u32> = Vec::new();
    for ch in chars {
        card_digits.push(ch.to_digit(10).expect("Error"));
    }
    let check: u32 = card_digits[card_digits.len() - 1];
    card_digits.remove(card_digits.len() - 1);
    for i in (0..card_digits.len()).step_by(2) {
        card_digits[i] = card_digits[i] * 2;
    }
    for i in 0..card_digits.len() {
        let value: String = card_digits[i].to_string();
        if value.len() == 1 {
            continue;
        } else {
            let vec: Vec<char> = value.chars().collect();
            let (x, y) = (vec[0], vec[1]);
            let sum: u32 = x.to_digit(10).expect("Error") + y.to_digit(10).expect("Error");
            card_digits[i] = sum;
        }
    }
    let digits_sum: u32 = card_digits.iter().sum::<u32>() * 9;
    let new_vec: Vec<char> = digits_sum.to_string().chars().collect();
    new_vec[new_vec.len() - 1].to_digit(10) == Some(check)
}
