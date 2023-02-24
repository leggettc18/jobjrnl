use clap::Args;
use clap::Parser;
use clap::Subcommand;
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
            let app = jobjrnl::JobApplication::new(
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
            list_applications(&db_pool).await?;
        }
    }
    Ok(())
}

async fn list_applications(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let recs = sqlx::query!(
        r#"
        SELECT id, name, date, resume_sent, coverletter_sent, response_date, interview_date
        FROM application
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        let app = jobjrnl::JobApplication::new(
            rec.name,
            Some(rec.date),
            rec.resume_sent,
            rec.coverletter_sent,
            rec.response_date,
            rec.interview_date,
        );
        println!("|{}| {}", rec.id, app);
    }

    Ok(())
}
