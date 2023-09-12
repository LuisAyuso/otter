use crate::model::{Coords, HalfField, HalfFieldLenght, LineWidth, Zone};

const ScrimmageLine: usize = HalfFieldLenght - 1;

#[derive(Debug)]
struct Solution;

impl Solution {
    fn new() -> Solution {
        Solution
    }
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

pub fn solve(hf: &HalfField) {
    println!("{}", hf.get_line(0)[0]);

    let los: Vec<Coords> = hf
        .get_line(ScrimmageLine)
        .iter()
        .enumerate()
        .filter(|(_, z)| z.is_player())
        .map(|(j, _)| (ScrimmageLine, j))
        .collect();

    let s = solve_for_play_1(hf, &los);
    println!("{:?}", s);
}
