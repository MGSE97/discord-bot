#[macro_export]
macro_rules! log_cmd {
    (
        $ctx:ident::$command:ident (
            $($arg:ident = $val:expr),*
        )
    ) => {
        {
            use ::colored::Colorize;
            use ::itertools::Itertools;
            let author = $ctx.author();
            ::tracing::info!(
                "\n#{user_id} @{user_name}: /{command} {args}",
                user_name = author.name.blue(),
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
            $($arg:ident = $val:expr),*
        ) => $response:expr
    ) => {
        {
            use ::colored::Colorize;
            use ::itertools::Itertools;
            let author = $ctx.author();
            ::tracing::info!(
                "\n#{user_id} @{user_name}: /{command} {args} => {response}",
                user_name = author.name.blue(),
                user_id = author.id.to_string().bright_black(),
                command = stringify!($command).green(),
                args = vec![
                    $((stringify!($arg), $val.yellow()),)*
                ].into_iter().map(|(k,v)|format!("{k}={v}")).join(" "),
                response = format!("{response:?}", response = $response).bright_black()
            );
        }
    };
}
