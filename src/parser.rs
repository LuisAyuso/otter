use std::{cell::RefCell, collections::BTreeMap, ops::Deref, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{alpha1, anychar, char, digit1, newline, space0, space1},
        is_alphabetic, is_newline, is_space,
    },
    combinator::{map, map_res},
    sequence, IResult,
};
use strum::IntoEnumIterator;

use crate::skills::Skill;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Player {
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
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Zone {
    Empty,
    Player(Player),
}
impl Zone {
    pub fn new_empty() -> Self {
        Zone::Empty
    }
    pub fn new_player(ident: char) -> Self {
        Zone::Player(Player::new(ident))
    }
}

#[derive(Default)]
struct Context {
    player_defs: BTreeMap<char, Player>,
}
impl Context {
    fn new() -> Context {
        Context {
            ..Default::default()
        }
    }

    fn add_player(&mut self, ident: char, player: Player) -> bool {
        self.player_defs.insert(ident, player).is_some()
    }
}

type Line = [Zone; 15];
type HalfField = [Line; 13];

fn zone(s: &str) -> IResult<&str, Zone> {
    map(alt((char(' '), anychar)), |c| match c {
        ' ' => Zone::Empty,
        ident => Zone::new_player(ident),
    })(s)
}

fn field_line(s: &str) -> IResult<&str, Line> {
    map(
        sequence::tuple((
            zone, zone, zone, zone, zone, zone, zone, zone, zone, zone, zone, zone, zone, zone,
            zone,
        )),
        |(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)| {
            [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]
        },
    )(s)
}

fn field(s: &str) -> IResult<&str, HalfField> {
    map(
        sequence::tuple((
            take_while(|c: char| is_newline(c as u8) || is_space(c as u8)),
            sequence::terminated(tag("==============="), newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            sequence::terminated(field_line, newline),
            tag("+++++++++++++++"),
        )),
        |(_, _, a, b, c, d, e, f, g, h, i, j, k, l, m, _)| [a, b, c, d, e, f, g, h, i, j, k, l, m],
    )(s)
}

fn skill(s: &str) -> IResult<&str, Skill> {
    map_res(
        take_while(|c| is_alphabetic(c as u8)),
        |skill_text: &str| match Skill::iter()
            .find(|skill| skill.to_string().to_lowercase() == skill_text.to_lowercase())
        {
            Some(x) => Ok(x),
            None => Err(nom::error::ErrorKind::Digit),
        },
    )(s)
}

fn strength(s: &str) -> IResult<&str, u8> {
    map_res(digit1, |s: &str| {
        s.parse::<u8>()
        // .map_err(|_| nom::error::ErrorKind::Fail)
    })(s)
}

fn skill_set(s: &str) -> IResult<&str, Vec<Skill>> {
    map_res(digit1, |s: &str| {
        s.parse::<u8>()
        // .map_err(|_| nom::error::ErrorKind::Fail)
    })(s)
}

/**
 * a player definition looks :
 * a: 4 guard fend block
 * b: 3
 */
fn player_def(ctx: Rc<RefCell<Context>>) -> impl Fn(&str) -> IResult<&str, ()> {
    move |s| {
        map_res(
            sequence::terminated(
                sequence::tuple((anychar, space0, tag(":"), space1, strength)),
                newline,
            ),
            |(ident, _, _, _, strength): (char, &str, &str, &str, u8)| {
                let mut ctx = ctx.deref().borrow_mut();
                match ctx.add_player(ident, Player::new(ident).with_strength(strength)) {
                    true => Ok(()),
                    false => Err(nom::error::ErrorKind::Fail),
                }
            },
        )(s)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse() {
        assert_eq!(zone("abc"), Ok(("bc", Zone::new_player('a'))));
        assert_eq!(zone("bc"), Ok(("c", Zone::new_player('b'))));
        assert_eq!(zone("c"), Ok(("", Zone::new_player('c'))));
        assert_eq!(zone(" c"), Ok(("c", Zone::new_empty())));

        assert_eq!(
            field_line(" aaa aaa aaa aa"),
            Ok((
                "",
                [
                    Zone::new_empty(),
                    Zone::new_player('a'),
                    Zone::new_player('a'),
                    Zone::new_player('a'),
                    Zone::new_empty(),
                    Zone::new_player('a'),
                    Zone::new_player('a'),
                    Zone::new_player('a'),
                    Zone::new_empty(),
                    Zone::new_player('a'),
                    Zone::new_player('a'),
                    Zone::new_player('a'),
                    Zone::new_empty(),
                    Zone::new_player('a'),
                    Zone::new_player('a'),
                ]
            ))
        );

        let half_field = r#"
        
        
===============
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
 aaa aaa aaa aa
+++++++++++++++
    "#;

        field(half_field).expect("ok");

        let ctx = Rc::new(RefCell::new(Context::new()));

        assert_eq!(skill("block"), Ok(("", Skill::Block)));
        assert_eq!(skill("Block"), Ok(("", Skill::Block)));
        assert_eq!(skill("Dodge"), Ok(("", Skill::Dodge)));
        assert_eq!(skill("Dodge "), Ok((" ", Skill::Dodge)));
    }
}
