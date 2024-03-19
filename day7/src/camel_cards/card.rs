use std::cmp::Ordering;

#[derive(Debug, Ord)]
pub enum CardType {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl PartialEq for CardType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for CardType {}

impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use CardType::*;

        Some(match (self, other) {
            (Ace, Ace) => Ordering::Equal,
            (Ace, _) => Ordering::Greater,
            (_, Ace) => Ordering::Less,

            (King, King) => Ordering::Equal,
            (King, _) => Ordering::Greater,
            (_, King) => Ordering::Less,

            (Queen, Queen) => Ordering::Equal,
            (Queen, _) => Ordering::Greater,
            (_, Queen) => Ordering::Less,

            (Jack, Jack) => Ordering::Equal,
            (Jack, _) => Ordering::Greater,
            (_, Jack) => Ordering::Less,

            (Ten, Ten) => Ordering::Equal,
            (Ten, _) => Ordering::Greater,
            (_, Ten) => Ordering::Less,

            (Nine, Nine) => Ordering::Equal,
            (Nine, _) => Ordering::Greater,
            (_, Nine) => Ordering::Less,

            (Eight, Eight) => Ordering::Equal,
            (Eight, _) => Ordering::Greater,
            (_, Eight) => Ordering::Less,

            (Seven, Seven) => Ordering::Equal,
            (Seven, _) => Ordering::Greater,
            (_, Seven) => Ordering::Less,

            (Six, Six) => Ordering::Equal,
            (Six, _) => Ordering::Greater,
            (_, Six) => Ordering::Less,

            (Five, Five) => Ordering::Equal,
            (Five, _) => Ordering::Greater,
            (_, Five) => Ordering::Less,

            (Four, Four) => Ordering::Equal,
            (Four, _) => Ordering::Greater,
            (_, Four) => Ordering::Less,

            (Three, Three) => Ordering::Equal,
            (Three, _) => Ordering::Greater,
            (_, Three) => Ordering::Less,

            (Two, Two) => Ordering::Equal,
            (Two, _) => Ordering::Greater,
            (_, Two) => Ordering::Less,

            (Joker, Joker) => Ordering::Equal,
        })
    }
}

#[derive(Debug)]
pub struct CardTypeParseFromCharError;

impl TryFrom<char> for CardType {
    type Error = CardTypeParseFromCharError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(CardTypeParseFromCharError),
        }
    }
}

#[derive(Debug, Ord)]
pub struct Card {
    pub label: char,
    pub card_type: CardType,
}

impl Card {
    pub fn new(label: char, joker_mode: bool) -> Self {
        let card_type = if joker_mode && label == 'J' {
            CardType::Joker
        } else {
            CardType::try_from(label).unwrap()
        };
        Card { label, card_type }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.card_type == other.card_type
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.card_type.partial_cmp(&other.card_type)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cards_are_ordered_correctly() {
        let ace_card = Card::new('A', false);
        let five_card = Card::new('5', false);
        assert_eq!(ace_card.partial_cmp(&five_card), Some(Ordering::Greater))
    }
}
