use serenity::{
    all::{CommandOptionType, Message},
    builder::{CreateCommand, CreateCommandOption},
    client::Context,
    framework::standard::{macros::command, CommandResult},
};

pub fn register() -> CreateCommand {
    CreateCommand::new("hoo")
        .description("Owl will listen to your inquery.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "inquery", "Inquery")
                .required(false),
        )
}

#[command]
#[description("Owl will listen to your inquery.")]
async fn hoo(ctx: &Context, msg: &Message) -> CommandResult {
    let text = msg.content.to_string();
    let response = respond_to_numbers(&text);
    msg.reply(ctx, &response).await?;

    Ok(())
}

pub fn respond_to_numbers(msg: &str) -> String {
    let mut response = "🦉 Hoo!".to_string();

    let numbers = get_msg_numbers(msg);

    for number in numbers {
        response = format!(
            "{response}\nZa {number} si {}",
            match number {
                p if p < 50 => "ani ptáčka nekoupíš.".to_string(),
                p if p < 15_000 => {
                    let birds = number / 50;
                    format!(
                        "koupíš {} {}.",
                        birds,
                        match birds {
                            1 => "ptáčka",
                            n if n <= 4 => "ptáčky",
                            _ => "ptáčků",
                        }
                    )
                }
                _ => {
                    let owls = number / 15_000;
                    format!(
                        "koupíš {} {}.",
                        owls,
                        match owls {
                            1 => "sovu",
                            n if n <= 4 => "sovy",
                            _ => "sov",
                        }
                    )
                }
            }
        );
    }

    response
}

fn get_msg_numbers(msg: &str) -> Vec<u64> {
    // Remove _ from numbers
    let mut cleaned = msg.to_lowercase().replace('_', "");

    // Fix dots
    if cleaned.contains('.') && cleaned.contains(',') {
        cleaned = cleaned.replace('.', "").replace(',', ".");
    } else if cleaned.contains(',') {
        cleaned = cleaned.replace(',', ".");
    }

    let mut chars = cleaned.chars().peekable();
    let mut numbers = vec![];
    let mut number_str = String::new();
    let mut has_dot = false;
    let mut has_exp = false;

    // Parse numbers
    while let Some(&c) = chars.peek() {
        // Add number parts
        if c.is_ascii_digit() || (c == '-' && number_str.is_empty()) {
            number_str.push(c);
        // Add dot
        } else if c == '.' && !has_dot {
            number_str.push(c);
            has_dot = true;
        }
        // Add exponent
        else if c == 'e' && !has_exp {
            number_str.push(c);
            has_exp = true;
        }
        // Parse number
        else {
            if let Some(parsed) = parse_number(&mut number_str, &mut chars) {
                numbers.push(parsed);
            }
            number_str = String::new();
            has_dot = false;
        }
        chars.next();
    }

    // Parse last number in message
    if let Some(parsed) = parse_number(&mut number_str, &mut chars) {
        numbers.push(parsed);
    }

    numbers
}

fn parse_number(
    number_str: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Option<u64> {
    let mut result = None;
    if !number_str.is_empty() {
        let number: Option<u64> = number_str
            .parse::<f64>()
            .ok()
            .and_then(|num| match num >= 0.0 {
                true => Some(num.floor() as u64),
                false => None,
            });
        if let Some(number) = number {
            let mut exponent = 0;
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() && exponent == 0 {
                    chars.next();
                    continue;
                }
                exponent = match c {
                    'k' => 3,
                    'm' => 6,
                    'g' => 9,
                    't' => 12,
                    'p' => 15,
                    'e' => 18,
                    'z' => 21,
                    'y' => 24,
                    'r' => 27,
                    'q' => 30,
                    _ => break,
                };
                chars.next();
                if let Some(&n) = chars.peek() {
                    if n.is_alphanumeric() {
                        exponent = 0;
                        break;
                    }
                }
            }
            result = match exponent > 0 {
                true => u64::checked_pow(10, exponent).and_then(|exp| number.checked_mul(exp)),
                false => Some(number),
            }
        }
    }
    result
}
