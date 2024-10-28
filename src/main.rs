use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let mut deck = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits{
        for rank in 1..=13{
            deck.push(Card{ suit, rank});
        }
    }

    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    let mut hand = Vec::new();
    for _ in 0..5{
        hand.push(deck.pop().unwrap());
    }
    hand.sort_by(|a,b| a.rank.cmp(&b.rank));

    println!("----- Hand -----");
    for (i, card) in hand.iter().enumerate(){
        println!("{} : {:?} {}", i+1, card.suit, card.rank);
    }

    println!("入れ替えたいカードを選択(ex:1 2 5)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    for number in numbers{
        hand[number -1 ] = deck.pop().unwrap();
    }

    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    for (i, card) in hand.iter().enumerate(){
        println!("{} : {:?} {}", i+1, card.suit, card.rank);
    }

    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit==suit);
    let mut count = 0;
    for i in 0..hand.len(){
        for j in i + 1..hand.len(){
            if hand[i].rank == hand[j].rank{
                count += 1;
            }
        }
    }

    // 簡略化
    if flash {
        println!("フラッシュ");
    } else if count >= 3 { 
        println!("スリーカード");
    } else if count == 2 {
        println!("２ペア");
    } else if count == 1 {
        println!("ワンペア");
    } else {
        println!("役なし");
    }
}
