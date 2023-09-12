use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{anychar, char, digit1, multispace0, multispace1, newline, space0, space1},
        is_alphabetic, is_newline, is_space,
    },
    combinator::{map, map_res},
    multi::{self, fold_many0},
    sequence, IResult,
};
use strum::IntoEnumIterator;

use crate::{
    model::{HalfField, Line, Player, Zone},
    skills::Skill,
};

#[derive(Default)]
struct ParseContext {
    player_defs: BTreeMap<char, Player>,
}
impl ParseContext {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn add_player(&mut self, ident: char, player: Player) -> anyhow::Result<()> {
        if self.player_defs.insert(ident, player).is_some() {
            anyhow::bail!("repeated player {}", ident);
        }
        Ok(())
    }

    fn add_players(&mut self, players: Vec<Player>) -> anyhow::Result<()> {
        for p in players {
            self.add_player(p.ident(), p)?;
        }
        Ok(())
    }

    fn get_player(&self, ident: char) -> Option<&Player> {
        self.player_defs.get(&ident)
    }
}

fn zone(ctx: Rc<RefCell<ParseContext>>) -> impl Fn(&str) -> IResult<&str, Zone> {
    move |s| {
        map_res(alt((char(' '), anychar)), |c| match c {
            ' ' => Ok(Zone::Empty),
            id => match ctx.borrow().get_player(id) {
                Some(p) => Ok(Zone::Player(p.clone())),
                None => {
                    println!("no player {} ", id);
                    Err(nom::error::ErrorKind::Fail)
                }
            },
        })(s)
    }
}

fn field_line(ctx: Rc<RefCell<ParseContext>>) -> impl Fn(&str) -> IResult<&str, Line> {
    move |s| {
        map(
            sequence::tuple((
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
                zone(ctx.clone()),
            )),
            |(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)| {
                [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]
            },
        )(s)
    }
}

fn half_field(ctx: Rc<RefCell<ParseContext>>) -> impl Fn(&str) -> IResult<&str, HalfField> {
    move |s| {
        map(
            sequence::tuple((
                take_while(|c: char| is_newline(c as u8) || is_space(c as u8)),
                sequence::terminated(tag("==============="), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                sequence::terminated(field_line(ctx.clone()), newline),
                tag("+++++++++++++++"),
            )),
            |(_, _, a, b, c, d, e, f, g, h, i, j, k, l, m, _)| {
                HalfField::new([a, b, c, d, e, f, g, h, i, j, k, l, m])
            },
        )(s)
    }
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
    map_res(digit1, |s: &str| s.parse::<u8>())(s)
}

fn skill_set(s: &str) -> IResult<&str, Vec<Skill>> {
    fold_many0(
        sequence::tuple((space0, skill)),
        Vec::new,
        |mut acc: Vec<Skill>, item| {
            acc.push(item.1);
            acc
        },
    )(s)
}

/**
 * a player definition looks :
 * a: 4 guard fend block
 * b: 3
 */
fn player_def(s: &str) -> IResult<&str, Player> {
    map(
        sequence::tuple((anychar, space0, tag(":"), space1, strength, skill_set)),
        |(ident, _, _, _, strength, skills): (char, _, _, _, u8, Vec<Skill>)| {
            Player::new(ident)
                .with_strength(strength)
                .with_skills(skills)
        },
    )(s)
}

fn parse_complete_field<'a>(
    ctx: Rc<RefCell<ParseContext>>,
) -> impl Fn(&'a str) -> IResult<&'a str, HalfField> {
    move |s| {
        let (s, _) = multispace0(s)?;

        let (s, players) = multi::many0(sequence::terminated(player_def, multispace1))(s)?;
        {
            let mut ctx = ctx.borrow_mut();
            ctx.add_players(players).expect("dup player");
        }

        let (s, _) = multispace0(s)?;

        let (s, hf) = half_field(ctx.clone())(s)?;

        Ok((s, hf))
    }
}

pub fn from_file(path: &std::path::Path) -> anyhow::Result<HalfField> {
    let s = std::fs::read_to_string(path).context("failed to read file")?;
    let ctx = Rc::new(RefCell::new(ParseContext::new()));
    let x = match parse_complete_field(ctx)(s.as_str()) {
        Ok((_, hf)) => Ok(hf),
        Err(_) => {
            // Here is a good place to print diagnosis, but we can not return further
            // because error reporting original string lifetime is bound to string.
            anyhow::bail!("compilation failed")
        }
    };
    x
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse() {
        let ctx = Rc::new(RefCell::new(ParseContext::new()));
        ctx.borrow_mut()
            .add_player('a', Player::new('a'))
            .expect("no dup");

        let hf = r#"
        
        
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

        half_field(ctx.clone())(hf).expect("ok");

        assert_eq!(skill("block"), Ok(("", Skill::Block)));
        assert_eq!(skill("Block"), Ok(("", Skill::Block)));
        assert_eq!(skill("Dodge"), Ok(("", Skill::Dodge)));
        assert_eq!(skill("Dodge "), Ok((" ", Skill::Dodge)));

        assert_eq!(skill_set(""), Ok(("", vec![])));
        assert_eq!(skill_set(" "), Ok((" ", vec![])));

        assert_eq!(
            skill_set("Dodge Block Accurate"),
            Ok(("", vec![Skill::Dodge, Skill::Block, Skill::Accurate]))
        );

        player_def("a: 3").expect("parses");
        player_def("a: 3 Block").expect("parses");
        player_def("a: 4 guard fend block").expect("hey!");

        let complete_def = r#"
a: 3 Block        
b: 4
        
===============
               
               
               
               
               
               
               
               
               
               
               
               
 a     b       
+++++++++++++++
    "#;

        println!("here we go!");

        let ctx = Rc::new(RefCell::new(ParseContext::new()));
        parse_complete_field(ctx)(complete_def).expect("rocks!");
    }
}
