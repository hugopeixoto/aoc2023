
trait Card {
    fn parse_card_v1(&self) -> usize;
    fn parse_card_v2(&self) -> usize;
}

impl Card for char {
    fn parse_card_v1(&self) -> usize {
        "__23456789TJQKA".find(*self).unwrap()
    }

    fn parse_card_v2(&self) -> usize {
        "_J23456789TQKA".find(*self).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
impl HType {
    pub fn new_v1(cards: &[usize; 5]) -> Self {
        let mut scards = [0; 16];
        for card in cards {
            scards[*card] += 1;
        }
        let mut counts = [0; 6];
        for c in scards {
            counts[c] += 1;
        }

        if counts[5] == 1 {
            Self::FiveOfAKind
        } else if counts[4] == 1 {
            Self::FourOfAKind
        } else if counts[3] == 1 && counts[2] == 1 {
            Self::FullHouse
        } else if counts[3] == 1 {
            Self::ThreeOfAKind
        } else if counts[2] == 2 {
            Self::TwoPair
        } else if counts[2] == 1 {
            Self::OnePair
        } else if counts[1] == 5 {
            Self::HighCard
        } else {
            panic!();
        }
    }

    pub fn new_v2(cards: &[usize; 5]) -> Self {
        let mut scards = [0; 16];
        for card in cards {
            scards[*card] += 1;
        }
        let mut counts = [0; 6];
        for c in &scards[2..] {
            counts[*c] += 1;
        }

        if counts[5 - scards[1]] >= 1 {
            Self::FiveOfAKind
        } else if counts[4 - scards[1]] >= 1 {
            Self::FourOfAKind
        } else if (counts[3] == 1 && counts[2] == 1) || (scards[1] == 1 && counts[2] == 2) { // J=1
            Self::FullHouse
        } else if counts[3 - scards[1]] >= 1 {
            Self::ThreeOfAKind
        } else if counts[2] == 2 {
            Self::TwoPair
        } else if counts[2 - scards[1]] >= 1 {
            Self::OnePair
        } else if counts[1] == 5 {
            Self::HighCard
        } else {
            panic!("unknown hand: {:?}", cards);
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    ttype: HType,
    cards: [usize; 5],
}

impl Hand {
    pub fn new_v1(s: &str) -> Self {
        let cards = s.chars().map(|c| c.parse_card_v1()).collect::<Vec<_>>().try_into().unwrap();
        let ttype = HType::new_v1(&cards);

        Hand { cards, ttype }
    }

    pub fn new_v2(s: &str) -> Self {
        let cards = s.chars().map(|c| c.parse_card_v2()).collect::<Vec<_>>().try_into().unwrap();
        let ttype = HType::new_v2(&cards);

        Hand { cards, ttype }
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut hands = input.lines().map(|line| {
        let mut parts = line.split(" ");
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse().unwrap();

        (Hand::new_v1(hand), bid)
    }).collect::<Vec<_>>();
    hands.sort();

    let p1 = hands.iter().enumerate().map(|(i, (_hand, bid))| (1 + i)*bid).sum();

    let mut hands = input.lines().map(|line| {
        let mut parts = line.split(" ");
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse().unwrap();

        (Hand::new_v2(hand), bid)
    }).collect::<Vec<_>>();
    hands.sort();

    let p2 = hands.iter().enumerate().map(|(i, (_hand, bid))| (1 + i)*bid).sum();
    (p1, p2)
}
