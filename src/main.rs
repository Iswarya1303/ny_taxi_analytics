use datafusion::arrow::util::pretty::print_batches;
use datafusion::error::Result;
use datafusion::functions_aggregate::expr_fn::{avg, count, sum};
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting NYC Taxi Analytics for 2025...\n");

    let ctx = SessionContext::new();

    // Load all 2025 yellow taxi parquet files
    let data_path = "data/yellow_tripdata_2025-*.parquet";

    ctx.register_parquet("trips", data_path, ParquetReadOptions::default())
        .await?;

    println!("All 2025 parquet files loaded successfully.\n");


    // Aggregation 1: Trips and revenue by month (SQL)
  
    println!("Aggregation 1 (SQL): Trips and revenue by month\n");

    let df1 = ctx
        .sql(
            "
            SELECT
                EXTRACT(MONTH FROM tpep_pickup_datetime) AS month,
                COUNT(*) AS trip_count,
                SUM(total_amount) AS total_revenue,
                AVG(fare_amount) AS avg_fare
            FROM trips
            GROUP BY month
            ORDER BY month
            ",
        )
        .await?;

    let results1 = df1.collect().await?;
    print_batches(&results1)?;


    // Aggregation 1: Trips and revenue by month (DataFrame API)

    println!("\nAggregation 1 (DataFrame API): Trips and revenue by month\n");

    let month_expr = date_part(lit("month"), col("tpep_pickup_datetime")).alias("month");

    let df_api1 = ctx
        .table("trips")
        .await?
        .aggregate(
            vec![month_expr],
            vec![
                count(lit(1)).alias("trip_count"),
                sum(col("total_amount")).alias("total_revenue"),
                avg(col("fare_amount")).alias("avg_fare"),
            ],
        )?
        .sort(vec![col("month").sort(true, true)])?;

    let results_api1 = df_api1.collect().await?;
    print_batches(&results_api1)?;


    // Aggregation 2: Tip behavior by payment type (SQL)
 
    println!("\nAggregation 2 (SQL): Tip behavior by payment type\n");

    let df2 = ctx
        .sql(
            "
            SELECT
                payment_type,
                COUNT(*) AS trip_count,
                AVG(tip_amount) AS avg_tip,
                SUM(tip_amount) / SUM(total_amount) AS tip_rate
            FROM trips
            GROUP BY payment_type
            ORDER BY trip_count DESC
            ",
        )
        .await?;

    let results2 = df2.collect().await?;
    print_batches(&results2)?;

  
    // Aggregation 2: Tip behavior by payment type (DataFrame API)

    println!("\nAggregation 2 (DataFrame API): Tip behavior by payment type\n");

    let df_api2 = ctx
        .table("trips")
        .await?
        .aggregate(
            vec![col("payment_type")],
            vec![
                count(lit(1)).alias("trip_count"),
                avg(col("tip_amount")).alias("avg_tip"),
                sum(col("tip_amount")).alias("sum_tip_amount"),
                sum(col("total_amount")).alias("sum_total_amount"),
            ],
        )?
        .select(vec![
            col("payment_type"),
            col("trip_count"),
            col("avg_tip"),
            (col("sum_tip_amount") / col("sum_total_amount")).alias("tip_rate"),
        ])?
        .sort(vec![col("trip_count").sort(false, true)])?;

    let results3 = df_api2.collect().await?;
    print_batches(&results3)?;

    println!("\nAll aggregations completed successfully.");

    Ok(())

}