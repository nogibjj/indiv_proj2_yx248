use indiv_proj2::{
    clear_table, connect_db, create_tables_if_not_exists, delete_data, insert_data,
    insert_data_from_csv, update_data,
}; // Import necessary functions
use mysql_async::prelude::Queryable;

#[tokio::test]
async fn test_table_creation() {
    let pool = connect_db();
    create_tables_if_not_exists(&pool).await;

    let mut conn = pool.get_conn().await.unwrap();
    let result_mini: Vec<String> = conn.query("SHOW TABLES LIKE 'week6_mini';").await.unwrap();
    assert!(!result_mini.is_empty(), "Table week6_mini not found");

    let result_discounts: Vec<String> = conn
        .query("SHOW TABLES LIKE 'week6_mini_discounts';")
        .await
        .unwrap();
    assert!(
        !result_discounts.is_empty(),
        "Table week6_mini_discounts not found"
    );
}

#[tokio::test]
async fn test_data_insert_csv() {
    let pool = connect_db();
    clear_table(&pool).await;
    insert_data_from_csv(&pool, "dataset_sample.csv").await;

    let mut conn = pool.get_conn().await.unwrap();
    let count: (i64,) = conn
        .query_first("SELECT COUNT(*) FROM week6_mini;")
        .await
        .unwrap()
        .expect("Expected a result from COUNT query");
    assert_eq!(count.0, 10, "Data insertion from CSV failed");
}

#[tokio::test]
async fn test_complex_query() {
    let pool = connect_db();
    let mut conn = pool.get_conn().await.unwrap();
    let results: Vec<(String, String, f64, i32, Option<f64>, f64)> = conn
        .query(
            r"SELECT w.Date, w.Product, w.Price, w.Quantity, d.Discount,
               (w.Price * w.Quantity) * IFNULL(1 - d.Discount, 1) AS Total_Revenue
        FROM week6_mini w
        LEFT JOIN week6_mini_discounts d ON w.Product = d.Product
        ORDER BY Total_Revenue DESC",
        )
        .await
        .unwrap();

    assert!(!results.is_empty(), "No results from the complex query");
}

#[tokio::test]
async fn test_data_insert() {
    let pool = connect_db();
    insert_data(&pool, "2022-01-01", "Banana", 0.5, 2).await;

    let mut conn = pool.get_conn().await.unwrap();
    let result: Vec<(String, String, f64, i32)> = conn
        .query("SELECT * FROM week6_mini WHERE Date = '2022-01-01' AND Product = 'Banana';")
        .await
        .unwrap();

    assert_eq!(result.len(), 1, "Data insertion failed");
    assert_eq!(result[0].2, 0.5, "Inserted price does not match");
    assert_eq!(result[0].3, 2, "Inserted quantity does not match");
}

#[tokio::test]
async fn test_data_update() {
    let pool = connect_db();
    insert_data(&pool, "2022-01-02", "Grape", 1.2, 5).await;
    update_data(&pool, "2022-01-02", "Grape", 1.5, 7).await;

    let mut conn = pool.get_conn().await.unwrap();
    let result: Vec<(String, String, f64, i32)> = conn
        .query("SELECT * FROM week6_mini WHERE Date = '2022-01-02' AND Product = 'Grape';")
        .await
        .unwrap();

    assert_eq!(result.len(), 1, "Data update failed");
    assert_eq!(result[0].2, 1.5, "Updated price does not match");
    assert_eq!(result[0].3, 7, "Updated quantity does not match");
}

#[tokio::test]
async fn test_data_delete() {
    let pool = connect_db();
    insert_data(&pool, "2022-01-03", "Orange", 0.8, 3).await;
    delete_data(&pool, "2022-01-03", "Orange").await;

    let mut conn = pool.get_conn().await.unwrap();
    let result: Vec<(String, String, f64, i32)> = conn
        .query("SELECT * FROM week6_mini WHERE Date = '2022-01-03' AND Product = 'Orange';")
        .await
        .unwrap();

    assert_eq!(result.len(), 0, "Data deletion failed");
}
