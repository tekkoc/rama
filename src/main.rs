use rand::prelude::*;
use std::cmp::Ordering;
use std::fmt;
use std::io::stdin;
use std::io::{stdout, Write};

const PLAYER_COUNT: u32 = 3;
const MAX_NUMBER: u32 = 6;
const FIRST_CARD_COUNT: u32 = 6;
const PER_CARD_COUNT: u32 = 8;
const RAMA_PENALTY: u32 = 10;
const BIG_POINT_TIP: u32 = 10;

#[derive(PartialEq, Eq, Ord)]
enum Card {
    Number(u32),
    Rama,
}

impl Card {
    fn new(n: u32) -> Self {
        match n {
            n if n == MAX_NUMBER + 1 => Card::Rama,
            n if 0 < n && n <= MAX_NUMBER => Card::Number(n),
            _ => panic!("failed create card"), // TODO error
        }
    }

    fn next(&self) -> Card {
        match self {
            Card::Number(n) if n == &MAX_NUMBER => Card::Rama,
            Card::Number(n) => Card::Number(n + 1),
            Self::Rama => Card::Number(1),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (l, r) if l == r => Some(Ordering::Equal),
            (_, Card::Rama) => Some(Ordering::Less),
            (Card::Rama, _) => Some(Ordering::Greater),
            (Card::Number(l), Card::Number(r)) if l < r => Some(Ordering::Less),
            (Card::Number(l), Card::Number(r)) if l > r => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Card::Number(n) => write!(f, "{}", n),
            Card::Rama => write!(f, "R"),
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    is_human: bool,
    point: u32,
    hands: Vec<Card>,
    is_folded: bool,
}

impl Player {
    fn new(name: String, is_human: bool) -> Self {
        Player {
            name,
            is_human,
            point: 0,
            hands: vec![],
            is_folded: false,
        }
    }

    fn reset(self: &mut Self) {
        // ポイント以外をリセットする
        self.hands = vec![];
        self.is_folded = false;
    }
}

#[derive(Debug)]
enum State {
    InGame,
    Result,
}

#[derive(Debug)]
struct Game {
    state: State,
    round: u32,
    deck: Vec<Card>,
    field: Vec<Card>,
    players: Vec<Player>,
    turn: u32,
}

impl Game {
    fn new() -> Self {
        let mut players = vec![];
        players.push(Player::new("Player".to_string(), true));
        for i in 0..PLAYER_COUNT - 1 {
            players.push(Player::new(format!("Npc{}", i + 1), false));
        }

        Game {
            state: State::InGame,
            round: 1,
            deck: vec![],
            field: vec![],
            players,
            turn: 0,
        }
    }

    fn start_round(self: &mut Self) {
        // 初期化
        self.deck = vec![];
        self.field = vec![];
        for p in &mut self.players {
            p.reset();
        }

        // デッキの作成
        for i in 1..=MAX_NUMBER + 1 {
            for _ in 0..PER_CARD_COUNT {
                self.deck.push(Card::new(i as u32))
            }
        }

        let mut rng = rand::thread_rng();
        self.deck.shuffle(&mut rng);

        self.turn = rng.gen_range(0..PLAYER_COUNT);

        // 場に1枚出す
        let card = self.deck.pop().unwrap();
        self.field.push(card);

        // 初期手札
        for i in 0..PLAYER_COUNT {
            let player = self.players.get_mut(i as usize).unwrap();

            for _ in 0..FIRST_CARD_COUNT {
                let card = self.deck.pop().unwrap();
                player.hands.push(card);
            }

            player.hands.sort();
        }
    }

    fn end_round(self: &mut Self) {
        for p in &mut self.players {
            // TODO 処理を移動する?

            // 手札が空ならポイントを減らす
            if p.hands.is_empty() {
                if p.point >= BIG_POINT_TIP {
                    p.point -= BIG_POINT_TIP;
                } else if p.point > 0 {
                    p.point -= 1;
                }
            } else {
                // ユニークにする(事前にソート済み)
                p.hands.dedup();
                p.point += p.hands.iter().fold(0, |sum, c| {
                    sum + match c {
                        Card::Number(n) => n,
                        Card::Rama => &RAMA_PENALTY,
                    }
                });
            }
        }

        let bottom_player = self
            .players
            .iter()
            .max_by(|p1, p2| p1.point.cmp(&p2.point))
            .unwrap();
        if bottom_player.point >= 40 {
            self.state = State::Result;
        } else {
            self.round += 1;

            self.start_round();
        }
    }

    fn get_turn_player(self: &Self) -> &Player {
        self.players.get(self.turn as usize).unwrap()
    }

    fn fold(self: &mut Self) {
        let player = self.players.get_mut(self.turn as usize).unwrap();
        player.is_folded = true;
    }

    fn draw(self: &mut Self) -> Option<()> {
        let player = self.players.get_mut(self.turn as usize)?;

        let card = self.deck.pop()?;
        player.hands.push(card);
        player.hands.sort();

        Some(())
    }

    fn play_card(self: &mut Self, target: String) -> Option<()> {
        let player = self.players.get_mut(self.turn as usize)?;

        let card = match target.as_str() {
            "1" | "2" | "3" | "4" | "5" | "6" => Some(Card::Number(target.parse().unwrap())),
            "l" | "r" | "L" | "R" => Some(Card::Rama),
            _ => None,
        }?;

        let top = self.field.last()?;

        if card != *top && card != top.next() {
            return None;
        }

        let index = player.hands.iter().position(|c| c == &card)?;
        player.hands.remove(index);
        self.field.push(card);

        Some(())
    }

    fn is_turn_end(self: &Self) -> bool {
        self.deck.is_empty()
            || self.players.iter().any(|p| p.hands.is_empty())
            || self.players.iter().all(|p| p.is_folded)
    }

    fn end_turn(self: &mut Self) {
        if self.is_turn_end() {
            self.end_round();
            return;
        }

        loop {
            self.turn += 1;
            if self.turn == PLAYER_COUNT {
                self.turn = 0;
            }

            let player = self.players.get_mut(self.turn as usize).unwrap();
            if !player.is_folded {
                break;
            }
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.start_round();

    loop {
        match game.state {
            State::InGame => {
                // TODO プレイヤー向けの表示をする

                let player = game.get_turn_player();
                if !player.is_human {
                    if let Some(_) = game.draw() {
                        game.end_turn();
                    }
                    continue;
                }

                println!("{:?}", game);

                println!("{} turn.", player.name);
                print!(">> ");
                stdout().flush().unwrap();

                let mut buffer = String::new();
                stdin()
                    .read_line(&mut buffer)
                    .expect("failed to read input");
                let command = buffer.as_str().trim();

                match command {
                    "exit" => {
                        println!("good bye!");
                        break;
                    }
                    "1" | "2" | "3" | "4" | "5" | "6" | "l" | "r" => {
                        if let Some(_) = game.play_card(command.to_string()) {
                            game.end_turn();
                        }
                    }
                    "d" => {
                        if let Some(_) = game.draw() {
                            game.end_turn();
                        }
                    }
                    "p" => {
                        game.fold();
                        game.end_turn();
                    }
                    _ => {}
                }
            }
            State::Result => {
                game.players.sort_by(|p1, p2| p1.point.cmp(&p2.point));

                for (i, p) in game.players.iter().enumerate() {
                    let rank = i + 1;
                    println!("{}. [{}]: -{}pt.", rank, p.name, p.point);
                }

                return;
            }
        }
    }
}
