use clap::Clap;
use dirs::data_dir;
use rusqlite::{params, Connection, Result};
use std::fs::create_dir_all;
use std::path::PathBuf;

#[derive(Clap)]
struct Opts {
    //#[clap(long)]
    //db: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Add,
    Show,
}

const TABLES: &str = "CREATE TABLE entry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT,
    amount REAL,
    project TEXT,
    description TEXT
);";

#[derive(Debug)]
struct Entry {
    pub id: i32,
    pub date: String,
    pub amount: f64,
    pub project: String,
    pub description: String,
}

impl Entry {
    pub fn new(date: &str, amount: f64, project: &str, description: &str) -> Self {
        Self {
            id: 0,
            date: String::from(date),
            amount,
            project: String::from(project),
            description: String::from(description),
        }
    }
}

struct Database {
    conn: Connection,
}

impl Database {
    pub fn init(db_file: PathBuf) -> Result<Self> {
        let dir = db_file.parent().unwrap();
        create_dir_all(dir).expect("Failed to create data directory");

        let create_tables = !db_file.exists();
        let conn = Connection::open(&db_file)?;
        if create_tables {
            conn.execute(TABLES, params![])?;
        }

        Ok(Self { conn })
    }

    pub fn insert(&self, entry: &Entry) -> Result<()> {
        self.conn.execute(
            "INSERT INTO entry (date, amount, project, description) VALUES (?1, ?2, ?3, ?4)",
            params![entry.date, entry.amount, entry.project, entry.description],
        )?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, date, amount, project, description FROM entry")?;

        let entry_iter = stmt.query_map(params![], |row| {
            Ok(Entry {
                id: row.get(0)?,
                date: row.get(1)?,
                amount: row.get(2)?,
                project: row.get(3)?,
                description: row.get(4)?,
            })
        })?;

        for entry in entry_iter {
            println!("{:?}", entry.unwrap());
        }

        Ok(())
    }
}

fn db_file() -> PathBuf {
    let mut db_file = PathBuf::new();
    db_file.push(data_dir().unwrap());
    db_file.push("com.tommyjl.rusty_tools");
    db_file.push("timesheet.db");
    db_file
}

fn show_cmd() -> Result<()> {
    let db = Database::init(db_file())?;
    db.get_all()?;
    Ok(())
}

fn add_cmd() -> Result<()> {
    let db = Database::init(db_file())?;
    db.insert(&Entry::new("2021-01-01", 1.0, "in:admin", "standup"))?;
    Ok(())
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Add => add_cmd(),
        SubCommand::Show => show_cmd(),
    }?;
    Ok(())
}
