use std::str::FromStr;

use anyhow::{anyhow, Context, Ok};

fn get_input() -> &'static str {
    "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let (x, y) = s
            .split_once(",")
            .ok_or_else(|| anyhow!(format!("unable to use {} text point", s)))?;
        let x: i32 = str::parse::<i32>(x).context("parse x")?;
        let y: i32 = str::parse::<i32>(y).context("parse y")?;
        return Ok(Point { x, y });
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn is_non_sloping(&self) -> bool {
        return self.p1.x == self.p2.x || self.p1.y == self.p2.y;
    }
}
impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let (x, y) = s
            .split_once(" -> ")
            .ok_or_else(|| anyhow!(format!("unable to use {} text line", s)))?;
        let p1 = str::parse(x).context("parse p1")?;
        let p2 = str::parse(y).context("parse p2")?;
        return Ok(Line { p1, p2 });
    }
}
fn main() {
    let res = get_input()
        .lines()
        .flat_map(str::parse::<Line>)
        .filter(|x| x.is_non_sloping())
        .collect::<Vec<_>>();

    println!("{:?}", res);
}
