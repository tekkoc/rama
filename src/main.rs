use rand::prelude::*;
use std::fmt;
use std::io::stdin;
use std::io::{stdout, Write};

const PLAYER_COUNT: u32 = 3;
const MAX_NUMBER: u32 = 6;
const FIRST_CARD_COUNT: u32 = 6;
const PER_CARD_COUNT: u32 = 8;

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
    point: u32,
    hands: Vec<Card>,
}

impl Player {
    fn new() -> Self {
        Player {
            point: 0,
            hands: vec![],
        }
    }
}

#[derive(Debug)]
struct Game {
    round: u32,
    deck: Vec<Card>,
    field: Vec<Card>,
    players: Vec<Player>,
    turn: u32,
}

impl Game {
    fn new() -> Self {
        let mut players = vec![];
        for _ in 0..PLAYER_COUNT {
            players.push(Player::new());
        }

        Game {
            round: 0,
            deck: vec![],
            field: vec![],
            players,
            turn: 0,
        }
    }

    fn start_round(self: &mut Self) {
        self.round += 1;

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
        }
    }

    fn play_card(self: &mut Self) {
        let player = self.players.get_mut(self.turn as usize).unwrap();

        // TODO 出す手札を指定するように
        // TODO ルール通りの手札しか出せないように
        // TODO 手札が空の場合

        let card = player.hands.pop().unwrap();
        self.field.push(card);
    }

    fn end_turn(self: &mut Self) {
        // TODO 終了判定
        // TODO ラウンドを降りているプレイヤーを飛ばす

        self.turn += 1;
        if self.turn == PLAYER_COUNT {
            self.turn = 0;
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.start_round();

    loop {
        println!("{:?}", game);

        // TODO プレイヤー向けの表示をする

        println!("select action. ");
        print!(">> ");
        stdout().flush().unwrap();

        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("failed to read input");

        match buffer.as_str().trim() {
            "exit" => {
                println!("good bye!");
                break;
            }
            "1" | "2" | "3" | "4" | "5" | "6" | "l" | "r" => {
                println!("play card!");
                game.play_card();
                game.end_turn();
            }
            "d" => {
                println!("draw!");
                game.end_turn();
            }
            "p" => {
                println!("pass!");
                game.end_turn();
            }
            _ => {}
        }
    }
}
