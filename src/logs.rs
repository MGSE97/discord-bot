/// Logs command execution details
///
/// Syntax:
///
/// ```ignore
/// log_cmd(
///     ctx::my_command(
///         // Args
///         input = input.unwrap_or("None")
///     ) => result
/// )
/// ```
#[macro_export]
macro_rules! log_cmd {
    (
        $ctx:ident::$command:ident (
            $($arg:ident = $val:expr),+
        )
    ) => {
        {
            use ::colored::Colorize;
            use ::itertools::Itertools;
            let author = $ctx.author();
            ::tracing::info!(
                "\n#{user_id}{user_nick} {alias}{user_name}: /{command} {args}",
                alias = "@".bright_black(),
                user_name = author.name.blue(),
                user_nick = author.global_name.clone().map(|nick| format!(" {alias}{nick}", alias = "@".bright_black())).unwrap_or_default().blue(),
                user_id = author.id.to_string().bright_black(),
                command = stringify!($command).green(),
                args = vec![
                    $((stringify!($arg), $val.yellow()),)*
                ].into_iter().map(|(k,v)| format!("{k}={v}")).join(" ")
            );
        }
    };
    (
        $ctx:ident::$command:ident (
            $($arg:ident = $val:expr),+
        ) => $response:expr
    ) => {
        {
            use ::colored::Colorize;
            use ::itertools::Itertools;
            let author = $ctx.author();
            ::tracing::info!(
                "\n#{user_id}{user_nick} {alias}{user_name}: /{command} {args} => {response}",
                alias = "@".bright_black(),
                user_name = author.name.blue(),
                user_nick = author.global_name.clone().map(|nick| format!(" {alias}{nick}", alias = "@".bright_black())).unwrap_or_default().blue(),
                user_id = author.id.to_string().bright_black(),
                command = stringify!($command).green(),
                args = vec![
                    $((stringify!($arg), $val.yellow()),)*
                ].into_iter().map(|(k,v)|format!("{k}={v}")).join(" "),
                response = format!("{response:?}", response = $response).bright_black()
            );
        }
    };
    (
        $ctx:ident::$command:ident $(())?
    ) => {
        {
            use ::colored::Colorize;
            let author = $ctx.author();
            ::tracing::info!(
                "\n#{user_id}{user_nick} {alias}{user_name}: /{command}",
                alias = "@".bright_black(),
                user_name = author.name.blue(),
                user_nick = author.global_name.clone().map(|nick| format!(" {alias}{nick}", alias = "@".bright_black())).unwrap_or_default().blue(),
                user_id = author.id.to_string().bright_black(),
                command = stringify!($command).green()
            );
        }
    };
    (
        $ctx:ident::$command:ident $(())? => $response:expr
    ) => {
        {
            use ::colored::Colorize;
            let author = $ctx.author();
            ::tracing::info!(
                "\n#{user_id}{user_nick} {alias}{user_name}: /{command} => {response}",
                alias = "@".bright_black(),
                user_name = author.name.blue(),
                user_nick = author.global_name.clone().map(|nick| format!(" {alias}{nick}", alias = "@".bright_black())).unwrap_or_default().blue(),
                user_id = author.id.to_string().bright_black(),
                command = stringify!($command).green(),
                response = format!("{response:?}", response = $response).bright_black()
            );
        }
    };
}
