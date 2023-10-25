use indiv_proj2::{
    check_data, clear_table, complex_query, connect_db, create_tables_if_not_exists, delete_data,
    insert_data, insert_data_from_csv, update_data,
};

#[tokio::main]
async fn main() {
    let pool = connect_db();
    create_tables_if_not_exists(&pool).await;
    clear_table(&pool).await;
    insert_data_from_csv(&pool, "dataset_sample.csv").await;
    complex_query(&pool).await;
    println!("\nChecking data:");
    check_data(&pool).await;

    println!("\nInserting data:");
    insert_data(&pool, "2020-01-01", "Apple", 1.0, 1).await;
    check_data(&pool).await;

    println!("\nUpdating data:");
    update_data(&pool, "2020-01-01", "Apple", 1.0, 3).await;
    check_data(&pool).await;

    println!("\nUpdating data:");
    update_data(&pool, "2020-01-01", "Apple", 2.0, 3).await;
    check_data(&pool).await;

    println!("\nDeleting data:");
    delete_data(&pool, "2020-01-01", "Apple").await;
    check_data(&pool).await;
}
