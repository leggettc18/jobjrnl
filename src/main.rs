use clap::Args;
use clap::Parser;
use clap::Subcommand;
use jobjrnl::CRUDable;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New(NewCmd),
    List(ListCmd),
}

#[derive(Args)]
struct ListCmd {}

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

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite:jobjrnl.sqlite?mode=rwc")
        .await?;
    sqlx::migrate!().run(&db_pool).await?;
    let cli = Cli::parse();
    match cli.command {
        Commands::New(cmd) => {
            let mut app = jobjrnl::JobApplication::new(
                cmd.name,
                cmd.date,
                cmd.resume_sent,
                cmd.coverletter_sent,
                cmd.response_date,
                cmd.interview_date,
            );
            app.create(&db_pool).await?;
            println!("{}", app)
        }
        Commands::List(..) => {
            let recs = jobjrnl::JobApplication::list(&db_pool).await?;
            for rec in recs {
                println!("{}", rec);
            }
        }
    }
    Ok(())
}
