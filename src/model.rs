use crate::skills::Skill;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Team {
    Defender,
    Attacker,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Player {
    team: Team,
    ident: char,
    strength: u8,
    skills: Vec<Skill>,
}
impl Player {
    pub fn new(ident: char) -> Self {
        Self {
            team: Team::Defender,
            ident,
            strength: 3,
            skills: vec![],
        }
    }
    pub fn with_strength(self, s: u8) -> Self {
        Self {
            team: self.team,
            ident: self.ident,
            strength: s,
            skills: self.skills,
        }
    }
    pub fn with_team(self, team: Team) -> Self {
        Self {
            team,
            ident: self.ident,
            strength: self.strength,
            skills: self.skills,
        }
    }
    pub fn with_skills(self, s: Vec<Skill>) -> Self {
        Self {
            team: self.team,
            ident: self.ident,
            strength: self.strength,
            skills: s,
        }
    }
    pub fn ident(&self) -> char {
        self.ident
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Zone {
    Out,
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
    pub fn is_player(&self) -> bool {
        matches!(self, Zone::Player(_))
    }
    pub fn is_empty(&self) -> bool {
        matches!(self, Zone::Empty)
    }
}
pub const LineWidth: usize = 15;
pub const HalfFieldLenght: usize = 13;

pub type Coords = (usize, usize);

pub type Line = [Zone; LineWidth];
pub struct HalfField([Line; HalfFieldLenght]);

impl HalfField {
    pub fn new(lines: [Line; HalfFieldLenght]) -> Self {
        HalfField(lines)
    }

    pub fn get_line(&self, i: usize) -> &Line {
        &self.0[i]
    }
    pub fn get_zone(&self, coords: &Coords) -> &Zone {
        &self.0[coords.0][coords.1]
    }

    /**
     * 0,1,2
     * 3,x,4
     * 5,6,7
     */
    pub fn get_neighbours(&self, coords: &Coords) -> [Zone; 8] {
        [
            Zone::Out,
            Zone::Out,
            Zone::Out,
            Zone::Out,
            Zone::Out,
            Zone::Out,
            Zone::Out,
            Zone::Out,
        ]
    }
}

impl std::fmt::Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Zone::Out => {}
            Zone::Empty => write!(f, " ")?,
            Zone::Player(p) => write!(f, "{}", p.ident())?,
        }
        Ok(())
    }
}

impl std::fmt::Display for HalfField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|===============|\n")?;
        for l in &self.0 {
            write!(f, "|")?;
            for z in l {
                write!(f, "{}", z)?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "|+++++++++++++++|\n")?;
        Ok(())
    }
}
