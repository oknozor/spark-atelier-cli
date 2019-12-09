use rand::seq::SliceRandom;
use std::io;
use std::io::stdin;
use std::io::Stdout;
use std::io::Write;
use std::str::Chars;
use std::thread;
use std::time::Duration;
use termion::color;
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::style;

const JUICE_MESSAGES: [&str; 4] = [
    "Bien jouer 👍!",
    "Excellent 👏!",
    "Vous m'impressionnez 👌!",
    "Congratulation 💪!",
];
const SHAME_MESSAGES: [&str; 3] = [
    "Shame, shame, shame 👎!",
    "Vous n'avez pas lu la consigne ?",
    "La prochaine fois, j'appelle Gilles Grimaud 👀",
];

type Terminal = MouseTerminal<RawTerminal<Stdout>>;

pub fn walkthrough() -> String {
    let mut term: Terminal = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    let atelier_1 = " _________                   __        _____   __         .__  .__".to_owned();

    let atelier_2 =
        "/   _____/__________ _______|  | __   /  _  \\_/  |_  ____ |  | |__| ___________"
            .to_owned();

    let atelier_3= "\\_____  \\\\____ \\__  \\\\_  __ \\  |/ /  /  /_\\  \\   __\\/ __ \\|  | |  |/ __ \\_  __ \\".to_owned();

    let atelier_4 =
        "/        \\  |_> > __ \\|  | \\/    <  /    |    \\  | \\  ___/|  |_|  \\  ___/|  | \\/"
            .to_owned();

    let atelier_5 =
        "/_______  /   __(____  /__|  |__|_ \\ \\____|__  /__|  \\___  >____/__|\\___  >__|"
            .to_owned();

    let atelier_6 =
        "        \\/|__|       \\/           \\/         \\/          \\/             \\/     "
            .to_owned();

    let lines = vec![
        atelier_1, atelier_2, atelier_3, atelier_4, atelier_5, atelier_6,
    ];

    lines.iter().for_each(|line| {
        print_delayed(line.chars(), Some(style::Bold), color::Green, 8, &mut term)
    });

    println!();
    println!();
    println!();

    let message = format!(
    "Bonjour {bold}{red}camarade{reset}, je suis foreman, ton contremaitre. Bienvenue dans mon atelier.",
        bold = style::Bold,
        red = color::Fg(color::Red),
        reset = style::Reset
    );

    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    let message = format!(
        "Aujourd'hui nous allons apprendre à utiliser Apache {bold}{red}S{blue}p{magenta}a{green}r{cyan}k{reset}",
            bold = style::Bold,
            red = color::Fg(color::Red),
            blue = color::Fg(color::Blue),
            magenta = color::Fg(color::Magenta),
            green = color::Fg(color::Green),
            cyan = color::Fg(color::Cyan),
            reset = style::Reset
        );

    print_delayed(message.chars(), None, color::Reset, 20, &mut term);
    println!();

    let message =
        "Au cours de cet atelier vous allez devoir accomplir une série d'exercices.".to_owned();
    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    let message = format!(
        "Pour valider un exercice appeler moi à l'aide de la commande {bold}{green}`./foreman next`{reset}.",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset = style::Reset
    );
    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    let message = format!(
        "{bold}{red}C'est la SEULE commande dont vous aurez besoin!{reset}",
        bold = style::Bold,
        red = color::Fg(color::Red),
        reset = style::Reset
    );
    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    print_delayed("Compris ?".chars(), None, color::Reset, 20, &mut term);

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('\n') => break,
            _ => {
                let position = term.cursor_pos().unwrap();
                let size = termion::terminal_size().unwrap();
                if size.1 == position.1 {
                    write!(term, "{}", termion::scroll::Up(1)).unwrap();
                }
                write!(
                    term,
                    "{}Si c'est bien compris presser entrée",
                    termion::cursor::Goto(1, position.1 + 1)
                )
                .unwrap();
            }
        }
    }

    println!();
    print_delayed("Parfait, pour valider un exercice vous aller devoir faire passer les tests unitaires de chaque exercice".chars(), None, color::Reset, 20, &mut term);
    print_delayed("Si un exercice est validé vous pourrez voir votre score augmenter sur le tableau des scores.".chars(), None, color::Reset, 20, &mut term);
    print_delayed(
        "En revanche, si vous tentez de me soumettre un exercice non valide".chars(),
        None,
        color::Reset,
        20,
        &mut term,
    );

    let message = format!(
        "je ne manquerai pas de vous couvrir de honte devant vos {bold}{red}camarades{reset}.",
        bold = style::Bold,
        red = color::Fg(color::Red),
        reset = style::Reset
    );

    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    println!();

    print_delayed("On résume :".chars(), None, color::Reset, 20, &mut term);

    let message = format!(
        "\t{bold}{green}✔️{reset} Implementer l'exercice demandé ",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset = style::Reset
    );

    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    let message = format!(
        "\t{bold}{green}✔️{reset} Utiliser la commande {bold}{green}`./foreman next`{reset}",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset = style::Reset
    );

    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    let message = format!(
        "\t{bold}{green}✔️{reset} Si tout c'est bien passer, je vous donnerai un nouvel exercice",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset = style::Reset
    );

    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    let message = format!(
        "\t{bold}{red}❌{reset}  Sinon c'est la honte et vous êtes bloqués tant que l'exercice n'est pas validé",
        bold = style::Bold,
        red = color::Fg(color::Red),
        reset = style::Reset
    );

    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    println!();
    print_delayed(
        "Avant de rentrer dans la compétition j'aurai besoin d'enregistrer le nom de votre équipe."
            .chars(),
        None,
        color::Reset,
        20,
        &mut term,
    );

    let message = format!(
        "Donnez moi un blaze qui claque, je veux du {bold}{pink}SWAG{reset} : ",
        bold = style::Bold,
        pink = color::Fg(color::Rgb(255, 51, 241)),
        reset = style::Reset
    );

    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    let team_name = type_team_name(&mut term);
    let message = format!(
        "\"{cyan}{bold}{team_name}{reset}\", personnellement j'aurai choisi un nom plus badass, on valide ? {bold}({green}O{reset}/{red}N{reset})",
        bold = style::Bold,
        green = color::Fg(color::Green),
        red = color::Fg(color::Red),
        cyan = color::Fg(color::Cyan),
        reset = style::Reset,
        team_name = &team_name
    );

    print_delayed(message.chars(), None, color::Reset, 20, &mut term);

    if validate_team_name(&team_name, &mut term) {
        return team_name.clone();
    }

    loop {
        print_delayed(
            "On change alors c'est quoi ce nom ?".chars(),
            None,
            color::Reset,
            20,
            &mut term,
        );

        let team_name = type_team_name(&mut term);

        let message = format!(
            "\"{bold}{cyan}{team_name}{reset}\", on valide ? {bold}({green}O{reset}/{red}N){reset}",
            bold = style::Bold,
            green = color::Fg(color::Green),
            red = color::Fg(color::Red),
            cyan = color::Fg(color::Cyan),
            reset = style::Reset,
            team_name = &team_name
        );
        print_delayed(message.chars(), None, color::Reset, 20, &mut term);

        if validate_team_name(&team_name, &mut term) {
            return team_name.clone();
        }
    }
}

pub fn shame() {
    let mut term: Terminal = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    print_delayed(
        SHAME_MESSAGES
            .choose(&mut rand::thread_rng())
            .unwrap()
            .chars(),
        None,
        color::Red,
        20,
        &mut term,
    );
}

pub fn congrat() {
    let mut term: Terminal = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    print_delayed(
        JUICE_MESSAGES
            .choose(&mut rand::thread_rng())
            .unwrap()
            .chars(),
        None,
        color::Green,
        20,
        &mut term,
    );
}

pub fn win() {
    let mut term: Terminal = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    let message = format!(
        "Félicitation {red}camarades{reset} vous avez terminé l'atelier!!!",
        red = color::Fg(color::Red),
        reset = style::Reset,
    );

    print_delayed(
        message.chars(),
        None,
        color::Reset,
        20,
        &mut term,
    );
}

fn print_delayed<T>(
    chars: Chars,
    bold: Option<style::Bold>,
    color: T,
    speed: u64,
    stdout: &mut Terminal,
) where
    T: color::Color,
{
    print!("{}", color::Fg(color));
    if let Some(bold) = bold {
        print!("{}", bold);
    }

    let position = stdout.cursor_pos().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, position.1)).unwrap();

    for c in chars {
        print!("{}", c);
        thread::sleep(Duration::from_millis(speed));
        stdout.flush().unwrap();
    }

    let position = stdout.cursor_pos().unwrap();
    let size = termion::terminal_size().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, position.1 + 1)).unwrap();
    if size.1 == position.1 {
        write!(stdout, "{}", termion::scroll::Up(1)).unwrap();
    }
    write!(stdout, "{}", termion::cursor::Goto(1, position.1 + 1)).unwrap();
    print!("{}", style::Reset);
}

fn type_team_name(term: &mut Terminal) -> String {
    let mut team_name = String::new();
    for c in stdin().keys() {
        // Input
        match c.unwrap() {
            Key::Char('\n') => break,
            Key::Backspace => {
                write!(
                    term,
                    "{} {}",
                    termion::cursor::Left(1),
                    termion::cursor::Left(1),
                )
                .unwrap();
                let _ = team_name.pop();
            }
            Key::Char(c) => {
                team_name.push(c);
                write!(term, "{}", c).unwrap()
            }
            _ => (),
        }
        term.flush().unwrap();
    }

    println!();

    team_name
}

fn validate_team_name(team_name: &String, term: &mut Terminal) -> bool {
    let mut validate = false;
    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('N') => {
                validate = false;
                break;
            }
            Key::Char('O') => {
                validate = true;
                break;
            }
            _ => {
                let message = format!(
                    "\"{bold}{cyan}{team_name}{reset}\", on valide ? {bold}({green}O{reset}/{red}N){reset}",
                    bold = style::Bold,
                    green = color::Fg(color::Green),
                    red = color::Fg(color::Red),
                    cyan = color::Fg(color::Cyan),
                    reset = style::Reset,
                    team_name = &team_name
                );
                print_delayed(message.chars(), None, color::Reset, 20, term);
            }
        }
    }
    validate
}
