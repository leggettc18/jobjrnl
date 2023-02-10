use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    //TODO: remove linter exception when more subcommands exist
    #[allow(clippy::single_match)]
    match cli.command {
        Commands::New(cmd) => {
            let app = jobjrnl::JobApplication::new(
                cmd.name,
                cmd.date,
                cmd.resume_sent,
                cmd.coverletter_sent,
                cmd.response_date,
                cmd.interview_date,
            );
            println!("{}", app)
        }
    }
}
