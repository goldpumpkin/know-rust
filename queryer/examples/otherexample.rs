use anyhow::Result;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/aronlindberg/latent_growth_classes/master/LGC_data.csv";

    // 使用 sql 从 URL 里获取数据
    let sql = format!(
        "SELECT top100_repository_name, month, monthly_increase, monthly_begin_at, monthly_end_with \
        FROM {} where monthly_increase >= 11 ORDER BY month DESC",
        url
    );

    let df1 = query(sql).await?;
    println!("{:?}", df1);

    Ok(())
}