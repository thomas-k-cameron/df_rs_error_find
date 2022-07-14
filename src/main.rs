use datafusion::{
    arrow::{
        array::{Array, ArrayRef, StringArray, UInt8Array},
    },
    common::DataFusionError,
    prelude::*,
};
use std::{fs::{self, File}, collections::HashMap};
use std::io::{BufRead, BufReader, Write};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let jpx = "./srnd-itch_20210324_D.csv";
    let sql = r#"
        SELECT 
            column_1,
            COUNT(column_1)
        FROM 
            csv
        GROUP BY
            column_1
    "#;
    let ctx = SessionContext::new();
    let opts = CsvReadOptions::new();
    ctx.register_csv("csv", "test1.csv", opts.delimiter(u8::MAX).has_header(false))
        .await
        .unwrap();

    let df = ctx.sql("SELECT * FROM csv").await.unwrap();
    df.show_limit(10).await.unwrap();

    let df = ctx.sql(sql).await.unwrap();
    df.show_limit(10).await.unwrap();

    //df.write_parquet("./example", None).await.unwrap();
    println!("Hello, world!");
}
