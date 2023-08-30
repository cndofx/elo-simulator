use std::fmt::Display;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Player {
    id: i32,
    score: f32,
    skill: i32,
    variation: i32,
    wins: u32,
    matches: u32,
}

impl Player {
    pub fn new(id: i32, score: f32, skill: i32, variation: i32) -> Self {
        Player {
            id,
            score,
            skill,
            variation,
            wins: 0,
            matches: 0,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn score(&self) -> f32 {
        self.score
    }

    pub fn skill_varied(&self) -> i32 {
        let mut rng = rand::thread_rng();
        self.skill + rng.gen_range(-self.variation..=self.variation)
    }

    pub fn add_match(&mut self) {
        self.matches += 1;
    }

    pub fn add_win(&mut self) {
        self.wins += 1;
    }

    pub fn add_score(&mut self, score: f32) {
        self.score += score;
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let win_percentage = if self.matches > 0 {
            (self.wins as f32) / (self.matches as f32) * 100.0
        } else {
            0.0
        };
        write!(
            f,
            "Player {:<3} | Score: {:<5.0} | Skill: {:<3}/{:>3} | Wins: {:<3} ({:.1}%)",
            self.id, self.score, self.skill, self.variation, self.wins, win_percentage
        )
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
