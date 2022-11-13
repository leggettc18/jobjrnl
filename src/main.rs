use chrono::Local;
use chrono::TimeZone;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New(NewCmd),
}

#[derive(Args)]
struct NewCmd {
    name: String,
    date: Option<String>,

    #[arg(long)]
    resume_sent: bool,

    #[arg(long)]
    coverletter_sent: bool,

    #[arg(long)]
    response_date: Option<String>,

    #[arg(long)]
    interview_date: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New(cmd)) => {
            let app = jobjrnl::JobApplication::new(
                cmd.name,
                cmd.date
                    .as_ref()
                    .map(|date| {
                        Local
                            .datetime_from_str(date.as_ref(), "%Y-%m-%d %H:%M:%S")
                            .unwrap()
                    })
                    .unwrap_or_else(Local::now),
                cmd.resume_sent,
                cmd.coverletter_sent,
                cmd.response_date.as_ref().map(|date| {
                    Local
                        .datetime_from_str(date.as_ref(), "%Y-%m-%d %H:%M:%S")
                        .unwrap()
                }),
                cmd.response_date.map(|date| {
                    Local
                        .datetime_from_str(date.as_ref(), "%Y-%m-%d %H:%M:%S")
                        .unwrap()
                }),
            );
            println!("{}", app)
        }
        None => {}
    }
}
