use std::{fs, process::exit};

use axum::{extract::Json as ExtractJson, http::HeaderMap, response::IntoResponse, routing::{get, post}, Json, Router};
use chrono::Utc;
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use axum::{
    http::StatusCode,
    routing::options,
};

#[derive(Deserialize)]
struct Config {
    api: Api,
}

#[derive(Deserialize)]
struct Api {
    ip: String,
}

// Utility function to create CORS headers
fn create_cors_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET,POST,OPTIONS,DELETE,PUT".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type".parse().unwrap());
    headers
}

fn cors_preflight() -> impl IntoResponse {
    let headers = create_cors_headers();
    (headers, StatusCode::NO_CONTENT)
}


// Define a struct to hold a row from the "sets" table
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Set {
    id: String,
    score1: i32,
    score2: i32,
    winner_name: String,
    date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NewSet {
    score1: i32,
    score2: i32,
    winner_name: String,
}

#[tokio::main]
async fn main() {
    // Create the database
    // Connect to the SQLite database
    let conn = Connection::open("sets.db").unwrap();

    let filename = "Config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };
    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

    // Create a new table named "sets" with the specified fields
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sets (
                  id TEXT PRIMARY KEY,
                  score1 INTEGER NOT NULL,
                  score2 INTEGER NOT NULL,
                  winner_name TEXT NOT NULL,
                  date TEXT NOT NULL
                  )",
        params![],
    ).unwrap();

    // build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/sets", get(|| async { get_all_sets_handler() }))
        .route("/add_set", post(move |new_set: ExtractJson<NewSet>| async move { add_set_handler(new_set) }))
        .route("/add_set", options(|| async { cors_preflight() }));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(config.api.ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_all_sets_handler() -> impl IntoResponse {
    let headers = create_cors_headers();
    let sets = get_all_sets();
    (headers, Json(sets))
}

fn add_set_handler(new_set: ExtractJson<NewSet>) -> impl IntoResponse {
    let headers = create_cors_headers();
    let set: Set = Set {
        id: Uuid::new_v4().to_string(),
        score1: new_set.score1,
        score2: new_set.score2,
        winner_name: new_set.winner_name.clone(),
        date: Utc::now().to_rfc3339(),
    };
    add_set(set.clone()).unwrap();
    (headers, Json(set))
}

fn add_set(set: Set) -> Result<(), rusqlite::Error> {
    // Connect to the SQLite database
    let conn = Connection::open("sets.db").unwrap();

    // Prepare the SQL statement
    conn.execute(
        "INSERT INTO sets (id, score1, score2, winner_name, date) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![set.id, set.score1, set.score2, set.winner_name, set.date],
    )?;

    Ok(())
}

// Function to get all sets from the database
fn get_all_sets() -> Vec<Set> {
    // Connect to the SQLite database
    let conn = Connection::open("sets.db").unwrap();

    // Prepare the SQL statement with explicit column types
    let mut stmt = conn.prepare("SELECT id, score1, score2, winner_name, date FROM sets").unwrap();

    // Query the database
    let sets_iter = stmt.query_map(params![], |row| {
        Ok(Set {
            id: row.get(0)?,
            score1: row.get(1)?,
            score2: row.get(2)?,
            winner_name: row.get(3)?,
            date: row.get(4)?,
        })
    }).unwrap();

    // Collect the results into a Vec
    let sets: Result<Vec<Set>, _> = sets_iter.collect();
    sets.unwrap()
}