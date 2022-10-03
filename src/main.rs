use std::fmt;
use std::io::stdin;
use std::io::{stdout, Write};

const PLAYER_COUNT: u32 = 3;
const MAX_NUMBER: u32 = 6;

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
    players: Vec<Player>,
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
            players,
        }
    }

    fn start_round(self: &mut Self) {
        self.round += 1;

        // デッキの作成
        for i in 1..=MAX_NUMBER + 1 {
            for _ in 0..8 {
                // TODO const
                self.deck.push(Card::new(i as u32))
            }
        }

        // TODO shuffle

        // 初期手札
        for i in 0..PLAYER_COUNT {
            for _ in 0..6 {
                // TODO const
                let card = self.deck.pop().unwrap();
                self.players.get_mut(i as usize).unwrap().hands.push(card);
            }
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.start_round();

    loop {
        println!("{:?}", game);

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
            }
            "d" => {
                println!("draw!");
            }
            "p" => {
                println!("pass!");
            }
            _ => {}
        }
    }
}
