use crate::skills::Skill;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Team {
    Defender,
    Attacker,
}

impl Team {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Attacker => Self::Defender,
            Self::Defender => Self::Attacker,
        }
    }
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
    pub fn strength(&self) -> u8 {
        self.strength
    }
    pub fn team(&self) -> Team {
        self.team
    }
    pub fn skills(&self) -> &[Skill] {
        &self.skills
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
pub const HalfFieldLength: usize = 13;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Coords(usize, usize);

impl Coords {
    pub fn new(i: usize, j: usize) -> Coords {
        Coords(i, j)
    }
    pub fn i(&self) -> usize {
        self.0
    }
    pub fn j(&self) -> usize {
        self.1
    }
}

impl std::ops::Add<Coords> for Coords {
    type Output = Coords;
    fn add(self, rhs: Coords) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Add<(isize, isize)> for Coords {
    type Output = Coords;
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        match (
            self.0.checked_add_signed(rhs.0),
            self.1.checked_add_signed(rhs.1),
        ) {
            (None, None) | (None, Some(_)) | (Some(_), None) => self,
            (Some(a), Some(b)) => Coords(a, b),
        }
    }
}

pub type Line = [Zone; LineWidth];
pub struct HalfField {
    lines: [Line; HalfFieldLength],
    out: Zone,
}

impl HalfField {
    pub fn new(lines: [Line; HalfFieldLength]) -> Self {
        HalfField {
            lines,
            out: Zone::Out,
        }
    }

    pub fn get_line(&self, i: usize) -> &Line {
        &self.lines[i]
    }
    pub fn get_zone(&self, coords: &Coords) -> &Zone {
        if coords.0 >= HalfFieldLength {
            &self.out
        } else if coords.1 >= LineWidth {
            &self.out
        } else {
            println!("{:?} {} {} ", coords, HalfFieldLength, LineWidth);
            &self.lines[coords.0][coords.1]
        }
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
        for l in &self.lines {
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
