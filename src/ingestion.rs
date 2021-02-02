use crate::models::{Move, Position, NewMove, NewPosition};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn ingest(db_name: &str, pgn_name: &str)  {
    embed_migrations!();

    println!("Ingesting {} into {}", pgn_name, db_name);

    let connection = SqliteConnection::establish(db_name)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_name));
    embedded_migrations::run(&connection);

    // Load PGN
    // parse
}

fn insert_position(conn: &SqliteConnection, fen: &str) -> Position {
    return Position {
        id: -1,
        fen: fen.to_string(),
    };
}

fn insert_move(conn: &SqliteConnection, san: &str, from: i32, to: i32) -> Move {
    return Move {
        id: 1,
        san: san.to_string(),
        from_id: -1,
        to_id: -1,
    };
}
