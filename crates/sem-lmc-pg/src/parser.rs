use crate::pg::{Node, Player, PG};

use std::io::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Parses a string which respects the following EBNF grammar:
///
/// 〈parity game〉 ::= [parity 〈identifier 〉 ;] 〈node spec〉+
/// 〈node spec〉 ::= 〈identifier 〉 〈priority〉 〈owner 〉 〈successors〉 [〈name〉] ;
/// 〈identifier 〉 ::= N
/// 〈priority〉 ::= N
/// 〈owner 〉 ::= 0 | 1
/// 〈successors〉 ::= 〈identifier 〉 (, 〈identifier 〉)∗
/// 〈name〉 ::= " ( any ASCII string not containing ‘"’) "
///
/// There is no error handling, a wrong input may parse and lead to weird
/// behaviour.
///
pub fn parse_pg(src: &mut BufReader<File>) -> Result<PG, Error> {
    src.lines().into_iter().skip(1).fold(Ok(PG(vec![])), |acc, elem| {
        match elem {
            Ok(str) if str.trim().is_empty() => acc,
            Ok(str) => {
                let str: Vec<&str> =
                    str.trim().split_whitespace().collect::<Vec<_>>();

                let node = Node {
                    owner: if str[2].parse::<u32>().unwrap() == 0 {
                        Player::Eve
                    } else {
                        Player::Adam
                    }, //
                    name: str[4]
                        .trim_end_matches("\";")
                        .trim_start_matches('"')
                        .trim()
                        .to_owned(),
                    id: str[0].parse::<u32>().unwrap(),
                    parity: str[1].parse::<u32>().unwrap(),
                };
                let adj_list = str[3]
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                acc.map(|mut pg| {
                    pg.0.push((node, adj_list));
                    pg
                })
            }
            Err(e) => Err(e),
        }
    })
}
