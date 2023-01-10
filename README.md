# CLI tool for generating luhn-valid card numbers

## Build

```shell
cd to/project/dir

cargo build --release
```

## Usage

```shell
cd to/builded/executable/file

./cardgen <CARD_LENGTH> <BIN> <NUMBER_OF_CARDS> <PATH>

# Where 
# CARD_LENGTH = length of card number
# BIN = first 6 digits of card
# NUMBER_OF_CARDS = How many cards to generate
# PATH = Path to the file where the cards will be saved

# For example:

./cardgen 16 532130 100000 ~/Desktop/cards.txt
```