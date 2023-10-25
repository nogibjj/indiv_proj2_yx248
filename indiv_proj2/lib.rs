extern crate dotenv;
extern crate futures;
extern crate mysql_async;

use csv::ReaderBuilder;
use mysql_async::prelude::*;
use mysql_async::*;
use std::fs::File;

pub mod config;
use config::get_db_config;

// connect to the database (connect)
pub fn connect_db() -> Pool {
    dotenv::dotenv().ok();
    let db_config = get_db_config();

    let connection_string = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_config.db_user,
        db_config.db_password,
        db_config.db_host,
        db_config.db_port,
        db_config.db_name
    );

    let opts = connection_string
        .parse::<mysql_async::Opts>()
        .expect("Invalid database configuration");
    Pool::new(opts)
}

// create tables if not exists (create)
pub async fn create_tables_if_not_exists(pool: &Pool) {
    let mut conn = pool.get_conn().await.unwrap();
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS week6_mini (
            Date DATE,
            Product VARCHAR(255),
            Price FLOAT,
            Quantity INT
        )",
    )
    .await
    .unwrap();

    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS week6_mini_discounts (
            Product VARCHAR(255),
            Discount FLOAT
        )",
    )
    .await
    .unwrap();
}

// clear the table (delete)
pub async fn clear_table(pool: &Pool) {
    let mut conn = pool.get_conn().await.unwrap();
    conn.query_drop(r"DELETE FROM week6_mini").await.unwrap();
}

// insert data from csv file (create)
pub async fn insert_data_from_csv(pool: &Pool, filepath: &str) {
    // Open the CSV file.
    let file = File::open(filepath).expect("Cannot open file");
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(file);

    // Prepare the insert query.
    let query = r"INSERT INTO week6_mini (Date, Product, Price, Quantity)
                  VALUES (?, ?, ?, ?)"; // Use ? placeholders

    for record in rdr.deserialize().flatten() {
        let (date, product, price, quantity): (String, String, f64, i32) = record;

        // Execute the insert query.
        let mut conn = pool.get_conn().await.expect("Failed to get connection");
        conn.exec_drop(
            query,
            (&date, &product, price, quantity), // Bind parameters in order
        )
        .await
        .expect("Failed to insert record");
    }
}

// function to run a complex query (read)
pub async fn complex_query(pool: &Pool) {
    let mut conn = pool.get_conn().await.unwrap();
    let result: Vec<(String, String, f64, i32, Option<f64>, f64)> = conn
        .query(
            r"SELECT w.Date, w.Product, w.Price, w.Quantity, d.Discount,
                   (w.Price * w.Quantity) * IFNULL(1 - d.Discount, 1) AS Total_Revenue
            FROM week6_mini w
            LEFT JOIN week6_mini_discounts d ON w.Product = d.Product
            ORDER BY Total_Revenue DESC",
        )
        .await
        .unwrap();

    for row in &result {
        println!("{:?}", row);
    }
}

// function to check all the data in the database (read)
pub async fn check_data(pool: &Pool) {
    let mut conn = pool.get_conn().await.unwrap();
    let result: Vec<(String, String, f64, i32)> = conn
        .query(
            r"SELECT Date, Product, Price, Quantity
            FROM week6_mini
            ORDER BY Date ASC",
        )
        .await
        .unwrap();

    for row in &result {
        println!("{:?}", row);
    }
}

// function to insert data into the database (create)
pub async fn insert_data(pool: &Pool, date: &str, product: &str, price: f64, quantity: i32) {
    let mut conn = pool.get_conn().await.unwrap();
    conn.exec_drop(
        r"INSERT INTO week6_mini (Date, Product, Price, Quantity)
        VALUES (?, ?, ?, ?)",
        (date, product, price, quantity),
    )
    .await
    .unwrap();
}

// function to delete data from the database (delete)
pub async fn delete_data(pool: &Pool, date: &str, product: &str) {
    let mut conn = pool.get_conn().await.unwrap();
    conn.exec_drop(
        r"DELETE FROM week6_mini
        WHERE Date = ? AND Product = ?",
        (date, product),
    )
    .await
    .unwrap();
}

// function to update data in the database (update)
pub async fn update_data(pool: &Pool, date: &str, product: &str, price: f64, quantity: i32) {
    let mut conn = pool.get_conn().await.unwrap();
    conn.exec_drop(
        r"UPDATE week6_mini
        SET Price = ?, Quantity = ?
        WHERE Date = ? AND Product = ?",
        (price, quantity, date, product),
    )
    .await
    .unwrap();
}
