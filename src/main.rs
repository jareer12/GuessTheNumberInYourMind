use rand::Rng;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn set_blue() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
    writeln!(&mut stdout, "")
}

fn set_normal() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(&mut stdout, "")
}

fn set_green() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "")
}

#[derive(Debug)]
struct GameTask {
    action: &'static str,
    number: u32,
}

fn input(msg: String) -> Option<String> {
    println!("{}", msg);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    return Some(input);
}

fn random() -> GameTask {
    let taskes: Vec<&str> = vec!["Increment", "Subtract"];
    let num = rand::thread_rng().gen_range(5..10);

    return GameTask {
        action: taskes[rand::thread_rng().gen_range(0..taskes.len())],
        number: num,
    };
}

fn main() {
    set_green();
    print!("How To Play?");
    set_normal();
    print!(
        "{}",
        "You will think of a number in your mind, then I will give you instruction to alter it, at last I will tell you what number you had in your mind."
    );

    println!();

    set_blue();
    print!("Welcome, I Will Guess Your Number");
    set_normal();
    let _guess = input(
        "Think of a number between 1 - 1000\nPress enter once you have thought of a number"
            .to_string(),
    )
    .unwrap();

    let mut _current: i32 = 0;
    let mut _times = 0;
    let _max: i8 = 5;

    loop {
        if _times == _max {
            // Magic, Trust Me
            println!("Now Remove The Number You Originally Chose");
            println!("The Number in Your Mind Is {}", _current);
            break;
        }

        let _data: GameTask = random();
        let _task = input(format!(
            "{} Your Number With {}. \nPress Enter To Continue",
            _data.action, _data.number
        ))
        .unwrap();
        if _data.action == "Increment" {
            _current = _current + _data.number as i32;
        } else if _data.action == "Subtract" {
            _current = _current - _data.number as i32;
        }
        _times = _times + 1;
        println!("{}", _current);
    }

    loop {}
}
