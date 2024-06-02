fn get_input() -> &'static str {
    return r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
}

#[derive(Debug)]
struct Movement {
    x: i32,
    y: i32,
}
fn parse_line(line: &str) -> Movement {
    let pair = line.split_once(" ");
    let pair = pair.expect(&format!("expected a pair in line {}", line));
    let (dir, amount) = pair;
    let amount = str::parse::<i32>(amount);
    let amount = amount.expect(&format!("2nd arg is not a number: {:?}", pair));

    match dir {
        "forward" => Movement { x: amount, y: 0 },
        "up" => Movement { x: 0, y: amount },
        _ => Movement { x: 0, y: -amount },
    }
}

fn main() {
    let get_input = &get_input();
    println!("{}", get_input);
    let lines = get_input.lines();
    let map = lines.map(parse_line);
    let point = map.fold(Movement { x: 0, y: 0 }, |mut acc, mv| {
        acc.x += mv.x;
        acc.y += mv.y;
        return acc;
    });
    println!("result: {:?}", point);
}
