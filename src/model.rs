use crate::skills::Skill;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Player {
    ident: char,
    strength: u8,
    skills: Vec<Skill>,
}
impl Player {
    pub fn new(ident: char) -> Self {
        Self {
            ident,
            strength: 3,
            skills: vec![],
        }
    }
    pub fn with_strength(self, s: u8) -> Self {
        {
            Self {
                ident: self.ident,
                strength: s,
                skills: self.skills,
            }
        }
    }
    pub fn with_skills(self, s: Vec<Skill>) -> Self {
        {
            Self {
                ident: self.ident,
                strength: self.strength,
                skills: s,
            }
        }
    }
    pub fn ident(&self) -> char {
        self.ident
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Zone {
    Empty,
    Player(Player),
}
impl Zone {
    pub fn new_empty() -> Self {
        Zone::Empty
    }
    pub fn new_player(player: Player) -> Self {
        Zone::Player(player)
    }
}

pub type Line = [Zone; 15];
pub type HalfField = [Line; 13];
