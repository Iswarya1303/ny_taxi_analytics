NYC Taxi Analytics with Rust and DataFusion
What this project does
Loads all 12 NYC TLC Yellow Taxi Parquet files for 2025 into DataFusion
Computes required aggregations using both DataFusion DataFrame API and DataFusion SQL
Prints readable aggregation results to the terminal
Stores a screenshot of the final output in the screenshots folder
Dataset source
NYC TLC Trip Record Data:
https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page

How to download data
Download all 12 monthly Yellow Taxi Parquet files for 2025 from the NYC TLC Trip Record Data page and place them in a local data/ folder.

Example filenames:

yellow_tripdata_2025-01.parquet
yellow_tripdata_2025-02.parquet
...
yellow_tripdata_2025-12.parquet
Note: Parquet files are not included in this repository.

How to run the project
cargo run


repo link -> https://github.com/Iswarya1303/ny_taxi_analytics
