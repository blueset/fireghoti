use crate::config::server::VERSION;

const GREETING_MESSAGE: &str = "\
███████╗██╗██████╗ ███████╗███████╗██╗███████╗██╗  ██╗    ○     ▄    ▄
██╔════╝██║██╔══██╗██╔════╝██╔════╝██║██╔════╝██║  ██║      ⚬   █▄▄  █▄▄
█████╗  ██║██████╔╝█████╗  █████╗  ██║███████╗███████║      ▄▄▄▄▄▄   ▄
██╔══╝  ██║██╔══██╗██╔══╝  ██╔══╝  ██║╚════██║██╔══██║     █      █  █▄▄
██║     ██║██║  ██║███████╗██║     ██║███████║██║  ██║     █ ● ●  █
╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝     ▀▄▄▄▄▄▄▀
 Firefish is an open-source decentralized microblogging platform.
 If you like Firefish, please consider contributing to the repo. https://firefish.dev/firefish/firefish
";

/// Prints the greeting message and the Firefish version to stdout.
#[macros::export]
pub fn greet() {
    println!("{}", GREETING_MESSAGE);

    tracing::info!("Welcome to Firefish!");
    tracing::info!("Firefish v{VERSION}");
}
