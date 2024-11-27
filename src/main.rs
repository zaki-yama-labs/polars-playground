use polars::prelude::*;

pub fn main() {
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("input/titanic3.csv".into()))
        .unwrap()
        .finish()
        .unwrap();
    let df2 = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("input/data1.csv".into()))
        .unwrap()
        .finish()
        .unwrap();
    let df3 = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("input/data1_2.csv".into()))
        .unwrap()
        .finish()
        .unwrap();
    let df3 = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("input/data1_3.csv".into()))
        .unwrap()
        .finish()
        .unwrap();
    println!("{}", df.head(Some(5)));
}
