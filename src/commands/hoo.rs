use poise::{
    command,
    serenity_prelude::{
        AutoArchiveDuration, Builder, ChannelType, CreateMessage, CreateThread, MessageId,
    },
};
use thousands::Separable;
use tracing::{error, instrument};

use crate::{log_cmd, Context, Error};

/// Owl will listen to your inquery.
#[instrument(level = "trace", skip(ctx), ret)]
#[command(prefix_command, slash_command)]
pub async fn hoo(
    ctx: Context<'_>,
    #[rest]
    #[description = "Inquery"]
    inquery: Option<String>,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id();
    // Check if command was invoked as slash command
    let message_id = if ctx.prefix() != "/" {
        // Get invocation channel and message id
        match ctx {
            poise::Context::Application(app) => app.interaction.data.target_id.map(MessageId::from),
            poise::Context::Prefix(pre) => Some(pre.msg.id),
        }
    } else {
        None
    };

    // Prepare and log response
    let response = respond_to_numbers(inquery.clone(), message_id.is_none());
    log_cmd!(
        ctx::hoo (
            inquery = inquery
                .map(|i| format!("{i:?}"))
                .unwrap_or_else(|| "None".to_string())
        ) => response
    );

    if let Some(message_id) = message_id {
        // Create thread and send reply there
        match CreateThread::new(format!(
            "🦉 Hoo {user_name}!",
            user_name = ctx.author().name
        ))
        .audit_log_reason("🦉 Hoo command response")
        .auto_archive_duration(AutoArchiveDuration::OneDay)
        .kind(ChannelType::PublicThread)
        .rate_limit_per_user(5)
        .invitable(true)
        .execute(ctx.http(), (channel_id, Some(message_id)))
        .await
        {
            Ok(thread) => {
                CreateMessage::new()
                    .content(&response)
                    .execute(ctx.http(), (thread.id, Some(thread.guild_id)))
                    .await?;
            }
            Err(err) => {
                error!("Failed to create thread for message {message_id} in {channel_id}: {err}");
                // Reply direcly, if we cant create thread
                // For instance in other thread
                ctx.reply(&response).await?;
            }
        }
    } else {
        // Reply direcly
        ctx.reply(&response).await?;
    }
    Ok(())
}

pub fn respond_to_numbers(inquery: Option<String>, include_message: bool) -> String {
    let message = inquery.clone().unwrap_or_default();
    let mut response = format!(
        "> 🦉 Hoo!{msg}",
        msg = match (include_message, inquery) {
            (true, Some(msg)) => format!("\n> {msg}"),
            _ => String::default(),
        }
    );

    if !message.trim().is_empty() {
        let numbers = get_msg_numbers(&message);

        for number in numbers {
            // Magic number to response transformer
            response = format!(
                "{response}\nZa {value} si {item}",
                value = number.separate_with_spaces(),
                item = match number {
                    0 => "∞ ptáčků užije.".to_string(),
                    7 => "devať nezkušiš.".to_string(),
                    9 => "kabel najdeš.".to_string(),
                    42 => "našel odpověď.".to_string(),
                    69 => "nice.".to_string(),
                    420 => "tě Michal najde.".to_string(),
                    p if (p & (p - 1)) == 0 => "binárků užiješ.".to_string(),
                    p if p < 50 => "ani ptáčka nekoupíš.".to_string(),
                    p if p < 15_000 => {
                        let birds = number / 50;
                        format!(
                            "koupíš {value} {item}.",
                            value = birds.separate_with_spaces(),
                            item = match birds {
                                1 => "ptáčka",
                                n if n <= 4 => "ptáčky",
                                _ => "ptáčků",
                            }
                        )
                    }
                    _ => {
                        let owls = number / 15_000;
                        format!(
                            "koupíš {value} {item}.",
                            value = owls.separate_with_spaces(),
                            item = match owls {
                                1 => "sovu",
                                n if n <= 4 => "sovy",
                                _ => "sov",
                            }
                        )
                    }
                }
            );
        }
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
    let mut has_esc = false;

    // Parse numbers
    while let Some(&c) = chars.peek() {
        // Add number parts
        if c == '<' || has_esc && c == '@' {
            has_esc = true;
        } else if c.is_ascii_digit() || (c == '-' && number_str.is_empty()) {
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
            if !(has_esc && matches!(chars.peek(), Some('>'))) {
                if let Some(parsed) = parse_number(&mut number_str, &mut chars) {
                    numbers.push(parsed);
                }
            }
            number_str = String::new();
            has_dot = false;
            has_esc = false;
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
        // Parse from string
        let number: Option<u64> = number_str
            .parse::<f64>()
            .ok()
            .and_then(|num| match num >= 0.0 {
                true => Some(num.floor() as u64),
                false => None,
            });
        if let Some(number) = number {
            // Parse optional exponent
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
            // Set resulting number
            result = match exponent > 0 {
                true => u64::checked_pow(10, exponent).and_then(|exp| number.checked_mul(exp)),
                false => Some(number),
            }
        }
    }
    result
}
