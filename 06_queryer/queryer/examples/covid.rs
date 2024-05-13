use anyhow::Result;
use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*};
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    //tracing_subscriber::fmt::init();
    let subscriber: fmt::Subscriber = fmt::Subscriber::builder()
    .with_max_level(tracing::Level::INFO)
    .finish();

    // 初始化 Tracing 日志框架的格式化输出，并使用上面创建的 subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let url: &str = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    // 使用 sql 从 URL 里获取数据
    let sql: String = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 10 ORDER BY new_cases DESC",
        url
    );
    let df1: queryer::DataSet = query(sql).await?;
    println!("{:?}", df1);

    Ok(())
}
