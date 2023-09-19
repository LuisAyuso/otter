use crate::model::{Coords, HalfField, HalfFieldLength, LineWidth, Player, Team, Zone};

const SCRIMMAGE_LINE: usize = HalfFieldLength - 1;

#[derive(Debug)]
struct Solution;

impl Solution {
    fn new() -> Solution {
        Solution
    }
}

fn get_neighbour_coords(center: &Coords) -> [Coords; 8] {
    [
        *center + (-1 as isize, -1 as isize),
        *center + (0 as isize, -1 as isize),
        *center + (1 as isize, -1 as isize),
        *center + (-1 as isize, 0 as isize),
        *center + (1 as isize, 0 as isize),
        *center + (-1 as isize, 1 as isize),
        *center + (0 as isize, 1 as isize),
        *center + (1 as isize, 1 as isize),
    ]
}

fn is_tackled(player: (&Coords, &Player), hf: &HalfField) -> bool {
    get_neighbours_of_team(player.0, player.1.team().opposite(), hf)
        .next()
        .is_some()
}

/**
 * 0,1,2
 * 3,x,4
 * 5,6,7
 */
fn get_neighbours_of_team<'a>(
    center: &Coords,
    team: Team,
    hf: &'a HalfField,
) -> impl std::iter::Iterator<Item = (Coords, &'a Player)> {
    get_neighbour_coords(center)
        .into_iter()
        .map(|coords| (coords, hf.get_zone(&coords)))
        .filter_map(|(coords, zone)| match zone {
            Zone::Out | Zone::Empty => None,
            Zone::Player(player) => Some((coords, player)),
        })
        .filter(move |(_, player)| player.team() == team)
}

fn block_odds(attacker: (&Coords, &Player), defender_coords: &Coords, hf: &HalfField) -> f32 {
    let (attacker_coords, attacker) = attacker;

    let defender = match hf.get_zone(defender_coords) {
        Zone::Out | Zone::Empty => panic!("block target not a player"),
        Zone::Player(p) => p,
    };

    let att_stg = attacker.strength() + get_supports(defender_coords, attacker.team(), hf) as u8;
    let def_stg = defender.strength() + get_supports(attacker_coords, defender.team(), hf) as u8;

    0.0
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Dice {
    ThreeAgainst,
    TwoAgainst,
    One,
    Two,
    Three,
}

fn get_dice(att_stg: u8, def_stg: u8) -> Dice {
    match (att_stg, def_stg) {
        (a, b) if a == b => Dice::One,
        (a, b) if b > a * 2 => Dice::ThreeAgainst,
        (a, b) if b > a => Dice::TwoAgainst,
        (a, b) if a > b * 2 => Dice::Three,
        (a, b) if a > b => Dice::Two,
        (_, _) => unreachable!(),
    }
}

fn get_supports(pos: &Coords, team: Team, hf: &HalfField) -> usize {
    get_neighbours_of_team(pos, team, hf)
        .filter(|(coords, player)| !is_tackled((&coords, player), hf))
        .count()
}

/// xxOxx
/// xPBxP
/// xxxPx
fn play_1(hf: &HalfField, center: &Coords) -> Solution {
    let i = center.i();
    let j = center.j();
    let first_blitz = (i, j - 1);
    let second_block = (i + 1, j + 2);
    let third_block = (i + 2, j + 1);
    let fourth_bock = (i + 1, j - 1);

    Solution::new()
}

fn solve_for_play_1(hf: &HalfField, los: &[Coords]) -> Vec<Solution> {
    los.iter()
        .filter(|c| c.j() >= 2 && c.j() + 2 < LineWidth)
        .map(|c| play_1(hf, c))
        .collect()
}

fn get_los(hf: &HalfField) -> Vec<Coords> {
    hf.get_line(SCRIMMAGE_LINE)
        .iter()
        .enumerate()
        .filter(|(_, z)| z.is_player())
        .map(|(j, _)| Coords::new(SCRIMMAGE_LINE, j))
        .collect()
}

pub fn solve(hf: &HalfField) {
    println!("{}", hf.get_line(0)[0]);
    let los = get_los(hf);
    let s = solve_for_play_1(hf, &los);
    println!("{:?}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::Team::Attacker, parser::from_str};

    #[test]
    fn play_1() {
        let one = include_str!("../demos/one.hf");
        let hf = from_str(one).expect("past gut");
        let los = get_los(&hf);
        assert_eq!(los.len(), 3);
    }
    #[test]
    fn dices() {
        for i in 1..6 {
            assert_eq!(get_dice(i, i), Dice::One);
        }
        for i in 1..6 {
            assert_eq!(get_dice(i, i + 1), Dice::TwoAgainst);
            assert_eq!(get_dice(i + 1, i), Dice::Two);
        }
        for i in 1..6 {
            assert_eq!(get_dice(i, i * 2), Dice::TwoAgainst);
            assert_eq!(get_dice(i * 2, i), Dice::Two);
        }
        for i in 1..6 {
            assert_eq!(get_dice(i, i * 2 + 1), Dice::ThreeAgainst);
            assert_eq!(get_dice(i * 2 + 1, i), Dice::Three);
        }
    }

    #[test]
    fn block() {
        let one = include_str!("../demos/two.hf");
        let hf = from_str(one).expect("past gut");

        let attacker = Player::new('1').with_team(Attacker);
        let in_front_of = Coords::new(SCRIMMAGE_LINE + 1, 6);
        let left_of = Coords::new(SCRIMMAGE_LINE + 1, 6 - 1);
        let right_of = Coords::new(SCRIMMAGE_LINE + 1, 6 + 1);

        let free = Coords::new(SCRIMMAGE_LINE + 2, 6 + 1);

        assert!(is_tackled((&in_front_of, &attacker), &hf));
        assert!(is_tackled((&left_of, &attacker), &hf));
        assert!(is_tackled((&right_of, &attacker), &hf));
        assert!(!is_tackled((&free, &attacker), &hf));

        block_odds(
            (&in_front_of, &attacker),
            &Coords::new(SCRIMMAGE_LINE, 6),
            &hf,
        );
    }
}
