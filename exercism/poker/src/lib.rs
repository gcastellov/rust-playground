use std::collections::HashMap;

#[derive(Eq, PartialOrd, Ord, PartialEq, Hash, Clone, Copy)]
enum Score {
    Pair = 1,
    DoublePair = 2,
    ThreeKind = 3,
    FiveHighStraight = 4,
    Straight = 5,
    Flush = 6,
    Full = 7,
    FourKind = 8,
    StraightFlush = 9,
    FiveKind = 10,
}

#[derive(Eq, PartialOrd, Ord, PartialEq, Clone, Copy)]
struct Card<'a> {
    position: usize,
    scoring: bool,
    as_str: &'a str,
}

#[derive(Eq, PartialOrd, Ord, PartialEq)]
struct Hand<'a> {
    original_hand: &'a str,
    sorted: Vec<Card<'a>>,
    score: Option<Score>,
    points: Option<usize>,
}

impl<'a> Hand<'a> {
    fn new(hand: &'a str) -> Self {
        let find_hits = || -> HashMap<&str, usize> {
            let mut kinds: HashMap<&str, usize> = HashMap::default();
            for card in hand.split(' ') {
                let value = &card[0..card.len() - 1];
                *kinds.entry(value).or_insert(0) += 1;
            }

            kinds
        };

        fn sort_hand<'a>(hand: &'a str, hits: HashMap<&str, usize>) -> Vec<Card<'a>> {
            let scores = |num: &str| -> bool { hits.len() == 5 || hits[num] > 1 };
            let sequence: Vec<&str> = vec![
                "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
            ];

            let mut sorted_hand: Vec<Card> = hand
                .split(' ')
                .map(|card| {
                    let num = &card[0..card.len() - 1];
                    Card {
                        position: sequence.iter().position(|c| *c == num).unwrap(),
                        scoring: scores(num),
                        as_str: card,
                    }
                })
                .collect();

            sorted_hand.sort();
            sorted_hand
        }

        fn score_hand(hits: &HashMap<&str, usize>, sorted: &Vec<Card>) -> Option<Score> {
            let is_straight = || -> bool {
                let mut straight: Vec<bool> = vec![false; 13];

                sorted
                    .iter()
                    .map(|card| card.position)
                    .for_each(|position| straight[position] = true);

                let lowest = sorted[0].position;
                (&straight[lowest..lowest + 5]).iter().all(|i| *i)
                    || (&straight[lowest..lowest + 4]).iter().all(|i| *i) && straight[12]
            };

            let is_same_suite = || -> bool {
                let mut suites = sorted
                    .iter()
                    .map(|card| {
                        let chars = card.as_str.chars().collect::<Vec<char>>();
                        *chars.last().unwrap()
                    })
                    .collect::<Vec<char>>();

                suites.sort();
                suites.dedup();
                suites.len() == 1
            };

            let mut hits_as_array: Vec<(&str, usize)> =
                hits.iter().map(|(c, h)| (*c, *h)).collect();
            hits_as_array.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
            hits_as_array.reverse();

            let (_, max_hit) = hits_as_array[0];
            let (_, next_hit) = hits_as_array[1];
            let last_position = sorted.last().unwrap().position;

            let same_suite = is_same_suite();

            match max_hit {
                1 => {
                    let straight = is_straight();
                    if straight && same_suite {
                        Some(Score::StraightFlush)
                    } else if straight && last_position == 12 {
                        Some(Score::FiveHighStraight)
                    } else if straight {
                        Some(Score::Straight)
                    } else if same_suite {
                        Some(Score::Flush)
                    } else {
                        None
                    }
                }
                2 => {
                    if next_hit == 2 {
                        Some(Score::DoublePair)
                    } else {
                        Some(Score::Pair)
                    }
                }
                3 => {
                    if next_hit == 2 {
                        Some(Score::Full)
                    } else {
                        Some(Score::ThreeKind)
                    }
                }
                4 => Some(Score::FourKind),
                5 => Some(Score::FiveKind),
                _ => None,
            }
        }

        let hits = find_hits();
        let sorted_hand = sort_hand(hand, hits.clone());
        let score = score_hand(&hits, &sorted_hand);
        let points = sorted_hand
            .iter()
            .filter(|card| card.scoring)
            .map(|card| card.position)
            .sum();

        Hand {
            original_hand: hand,
            sorted: sorted_hand,
            score: score,
            points: Some(points),
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut scoring_hands = get_scoring_hands(hands);
    if !scoring_hands.is_empty() {
        scoring_hands.sort_by(|a, b| a.points.cmp(&b.points));
        let max_score = scoring_hands.iter().map(|hand| hand.points).max().unwrap();
        let candidates: Vec<&Hand> = scoring_hands
            .iter()
            .filter(|hand| hand.points == max_score)
            .map(|hand| hand)
            .collect();

        if candidates.len() > 1 {
            return Some(get_winners(scoring_hands));
        } else {
            return Some(candidates.iter().map(|hand| hand.original_hand).collect());
        }
    }

    let all_hands: Vec<Hand> = hands.iter().map(|h| Hand::new(h)).collect();

    Some(get_winners(all_hands))
}

fn get_scoring_hands<'a>(hands: &[&'a str]) -> Vec<Hand<'a>> {
    let mut sorted_hands: Vec<Hand<'a>> = hands.iter().map(|h| Hand::new(h)).collect();
    
    sorted_hands.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    sorted_hands.reverse();

    let highest_score = sorted_hands[0].score;
    sorted_hands
        .into_iter()
        .filter(|hand| hand.score.is_some())
        .filter_map(|hand| {
            if hand.score.unwrap() == highest_score.unwrap() {
                Some(hand)
            } else {
                None
            }
        })
        .collect()
}

fn get_winners<'a>(hands: Vec<Hand<'a>>) -> Vec<&'a str> {
    let positions: Vec<Vec<usize>> = hands
        .iter()
        .map(|hand| hand.sorted.iter().map(|card| card.position).collect())
        .collect();

    let winner = order_them_and_win(positions);

    let result: Vec<&'a str> = hands
        .iter()
        .filter_map(|hand| {
            let mut set: Vec<_> = hand.sorted.iter().map(|card| card.position).collect();
            set.reverse();

            if set == winner {
                Some(hand.original_hand)
            } else {
                None
            }
        })
        .collect();

    result
}

fn order_them_and_win(all: Vec<Vec<usize>>) -> Vec<usize> {
    let mut new_all: Vec<Vec<usize>> = all
        .into_iter()
        .map(|hand| {
            let mut h = Vec::from(hand);
            h.sort();
            h.reverse();
            h
        })
        .collect();

    new_all.sort_by(|a, b| a.cmp(&b));
    new_all.reverse();
    new_all[0].clone()
}
