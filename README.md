# NYC Taxi Analytics 

## What the Project Does

- Loads all NYC TLC Yellow Taxi trip data for the **year 2025** from Parquet files into the DataFusion query engine.
- Performs analytical aggregations using **DataFusion SQL** and the **DataFusion DataFrame API**.
- Computes monthly trip statistics and tip behavior by payment type.
- Displays the results as formatted tables in the terminal.

---

## Dataset Source

NYC TLC Trip Record Data  
https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page

Data Dictionary  
https://www.nyc.gov/assets/tlc/downloads/pdf/data_dictionary_trip_records_yellow.pdf

---

## How to Download Data

Download the **Yellow Taxi Trip Records for 2025** from the NYC TLC website.

Place the downloaded files in a local directory named:

---

## Aggregation 1 – Trips and Revenue by Month

This aggregation groups taxi trips by the **pickup month** derived from `tpep_pickup_datetime`.

It calculates:

- Total number of trips
- Total revenue using `total_amount`
- Average fare using `fare_amount`

Results are sorted by month in ascending order.

---

## Aggregation 2 – Tip Behavior by Payment Type

This aggregation groups trips by **payment_type**.

It calculates:

- Trip count
- Average tip amount (`tip_amount`)
- Tip rate defined as total tips divided by total revenue.

Results are sorted by trip count in descending order.

"repo link" -> https://github.com/Iswarya1303/ny_taxi_analytics
