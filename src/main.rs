use clap::{Arg, Command};

mod client;
mod server;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .parse_env("LOG")
        .init();

    let matches = Command::new("Obirt")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            Command::new("client")
                .about("Connect to a server")
                .arg(
                    Arg::new("address")
                        .help("Server address")
                        .value_name("ADDRESS")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new("port")
                        .long("port")
                        .short('p')
                        .help("Server port")
                        .value_name("PORT")
                        .default_value("9807"),
                )
                .arg(
                    Arg::new("interface")
                        .long("interface")
                        .short('i')
                        .help("Interface name")
                        .value_name("INTERFACE")
                        .default_value("obr0"),
                ),
        )
        .subcommand(Command::new("server").about("Start a server"))
        .subcommand(Command::new("info").about("Show information about the connection"))
        .get_matches();

    match matches.subcommand() {
        Some(("client", command)) => {
            let server = command.get_one::<String>("address").unwrap();
            let port = command.get_one::<String>("port").unwrap();
            let interface = command.get_one::<String>("interface").unwrap();

            client::connect(server, port, interface).await;
        }
        Some(("server", _)) => {
            server::serve().await;
        }
        _ => {
            println!("No subcommand was used");
        }
    }
}
