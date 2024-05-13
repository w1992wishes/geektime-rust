extern crate polars;
use polars::prelude::*;
use std::fs::File;

fn main() {
    let mut df: DataFrame = DataFrame::default();

    // 增加列
    df.with_column(Series::new("榜号", vec![1, 2, 3]));
    df.with_column(Series::new("姓名", vec!["张三丰", "黄老邪", "老顽童"]));

    println!("{:?}", df);

    let x = df.column("榜号").unwrap();
    println!("{:?}",x);

    let q: Series = (0..100).map(|x: i32| (2 * x) as i32).collect();
    println!("{:?}", q);
}
