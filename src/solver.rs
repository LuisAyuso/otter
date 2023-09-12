use crate::model::{Coords, HalfField, HalfFieldLenght, LineWidth, Player, Zone};

const ScrimmageLine: usize = HalfFieldLenght - 1;

#[derive(Debug)]
struct Solution;

impl Solution {
    fn new() -> Solution {
        Solution
    }
}

fn block_odds(attacker: (Player, &Coords), victim_coords: &Coords, hf: &HalfField) -> f32 {
    let victim = match hf.get_zone(victim_coords) {
        Zone::Empty => panic!("block target not a player"),
        Zone::Player(p) => p,
    };

    // get players tackling attacker
    

    0.0
}

/// xxOxx
/// xPBxP
/// xxxPx
fn play_1(hf: &HalfField, center: &Coords) -> Solution {
    let (i, j) = center;
    let first_blitz = (i, j - 1);
    let second_block = (i + 1, j + 2);
    let third_block = (i + 2, j + 1);
    let fourth_bock = (i + 1, j - 1);

    Solution::new()
}

fn solve_for_play_1(hf: &HalfField, los: &[Coords]) -> Vec<Solution> {
    los.iter()
        .filter(|(_, j)| j >= &2 && j + 2 < LineWidth)
        .map(|c| play_1(hf, c))
        .collect()
}

fn get_los(hf: &HalfField) -> Vec<Coords> {
    hf.get_line(ScrimmageLine)
        .iter()
        .enumerate()
        .filter(|(_, z)| z.is_player())
        .map(|(j, _)| (ScrimmageLine, j))
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

    use crate::parser::from_str;

    use super::*;

    #[test]
    fn play_1() {
        let one = include_str!("../demos/one.hf");
        let hf = from_str(one).expect("past gut");
        let los = get_los(&hf);
    }
}
