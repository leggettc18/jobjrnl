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
            let _id = add_application(&db_pool, &app).await?;
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
        "#
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

async fn add_application(
    pool: &sqlx::SqlitePool,
    app: &jobjrnl::JobApplication,
) -> Result<i64, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        r#"
        INSERT INTO application (name, date, resume_sent, coverletter_sent, response_date, interview_date)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6);
        "#,
        app.name, app.date_applied, app.resume_sent, app.coverletter_sent, app.response_received, app.interview_date
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}
