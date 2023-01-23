// lifo stack of stuff I find interesting
// id:
// time:
// url:
// category:  book, comic, film, tv, music
// name:
// meta:
//
// usage
// stack
// stack -c book
// stack -i 23
// stack add -c book -n Mason & Dixon -m Thomas Pynchon
//
// stack edit -i 23 -n Gravity's Rainbow

#[macro_use] extern crate prettytable;
use rusqlite::{Connection, Row, Result};
use clap::{Parser, Subcommand};
use prettytable::{Table, Row as Prow};

#[derive(Debug)]
struct Item {
    id: i32,
    category: String,
    name: String,
    meta: Option<String>,
}

impl Item {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Item {
            id: row.get_unwrap(0),
            category: row.get_unwrap(1),
            name: row.get_unwrap(2),
            meta: row.get_unwrap(3),
        })
    }

    pub fn to_prow(&self) -> Prow {
        row![self.id, self.category, self.name, self.meta.as_deref().unwrap_or("")]
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// create sqlite db
    Create,

    /// add new entry
    Add {
        /// book, comic, film, tv, music
        #[arg(short)]
        category: String,

        /// what it called
        #[arg(short)]
        name: String,

        /// author, artist
        #[arg(short)]
        meta: Option<String>,
    },
}

fn get_conn() -> Connection {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{home}/.stack.db");
    Connection::open(path).unwrap()
}

fn create(conn: Connection) {
    conn.execute(
        "CREATE TABLE items (
            id   INTEGER PRIMARY KEY,
            category TEXT NOT NULL,
            name TEXT NOT NULL,
            meta TEXT,
            time DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (), // empty list of parameters.
    ).unwrap();
}

fn list(conn: Connection) {
    let mut stmt = conn.prepare("SELECT id, category, name, meta FROM items").unwrap();
    let items_iter = stmt.query_map([], Item::from_row).unwrap();

    let mut table = Table::new();
    table.add_row(row!["ID", "CATEGORY", "NAME", "META"]);

    for item in items_iter {
        table.add_row(item.unwrap().to_prow());
    }

    table.printstd();
}

fn main() {
    let cli = Cli::parse();
    let conn = get_conn();

    match &cli.command {
        Some(Commands::Create) => create(conn),
        Some(Commands::Add { category, name, meta }) => {
            conn.execute(
                "INSERT INTO items (category, name, meta) VALUES (?1, ?2, ?3)",
                (category, name, meta),
            ).unwrap();
        }
        None => {
            list(conn);
        }
    }
}
