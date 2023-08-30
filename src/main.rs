use std::fmt::Debug;

use elo_rating_simulator::get_two_mut;
use rand::Rng;

use player::Player;

mod player;

const MAX_INCREASE: f32 = 32.0;
const INITIAL_SCORE: f32 = 1500.0;

#[derive(Debug)]
struct State {
    players: Vec<Player>,
}

impl State {
    fn new(players: usize) -> Self {
        let mut rng = rand::thread_rng();
        let players: Vec<Player> = (0..players)
            .map(|id| {
                Player::new(
                    id as i32,
                    INITIAL_SCORE,
                    rng.gen_range(0..=99),
                    rng.gen_range(0..=99),
                )
            })
            .collect();
        State { players }
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn random_players(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let players = self.players.len();
        loop {
            let p1 = rng.gen_range(0..players);
            let p2 = rng.gen_range(0..players);
            if p1 != p2 {
                return (p1, p2);
            }
        }
    }

    fn print_rankings(&self) {
        // sort players by score in descending order
        let mut players = self.players.clone();
        players.sort_by(|p1, p2| p2.score().partial_cmp(&p1.score()).unwrap());

        for (i, player) in players.iter().enumerate() {
            println!("{:>4}. {}", i + 1, player);
        }
    }

    fn simulate_match(&mut self, p1: usize, p2: usize, print: bool) {
        assert_ne!(p1, p2, "players must not be the same");
        let (p1, p2) = get_two_mut(&mut self.players, p1, p2);

        let exp1 = (p2.score() - p1.score()) / 400.0;
        let exp2 = (p1.score() - p2.score()) / 400.0;

        let e1 = MAX_INCREASE * 1.0 / (1.0 + 10f32.powf(exp1));
        let e2 = MAX_INCREASE * 1.0 / (1.0 + 10f32.powf(exp2));
        let s1 = p1.skill_varied();
        let s2 = p2.skill_varied();

        if print {
            println!("{p1}");
            println!("  vs");
            println!("{p2}");
        }

        p1.add_match();
        p2.add_match();
        if s1 > s2 {
            p1.add_win();
            p1.add_score(e2);
            p2.add_score(-e2);
            if print {
                print_match_results(&p1, &p2, e2);
            }
        } else {
            p2.add_win();
            p1.add_score(-e1);
            p2.add_score(e1);
            if print {
                print_match_results(&p2, &p1, e1);
            }
        }

        fn print_match_results(winner: &Player, loser: &Player, elo_delta: f32) {
            println!(
                "Player {} won! Gained {:.0} elo ({:.0} -> {:.0}).",
                winner.id(),
                elo_delta,
                winner.score() - elo_delta,
                winner.score()
            );
            println!(
                "Player {} lost. Lost {:.0} elo ({:.0} -> {:.0}).",
                loser.id(),
                elo_delta,
                loser.score() + elo_delta,
                loser.score()
            );
            println!();
        }
    }
}

fn main() {
    let mut state = State::new(100);
    for _ in 0..100000 {
        let (p1, p2) = state.random_players();
        state.simulate_match(p1, p2, false);
    }
    state.print_rankings();
}
