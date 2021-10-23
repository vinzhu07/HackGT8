#![allow(unused)]

#[macro_use]
extern crate diesel;

use std::error::Error;
use std::io;
use std::fs::File;
use std::process;

use diesel::prelude::*;
use diesel::Queryable;

use serde::Deserialize;

use diesel::prelude::*;
use diesel::sqlite;
use diesel::sqlite::SqliteConnection;
mod schema;
use schema::clothes;

#[derive(Debug, Deserialize)]
struct CsvStyles {
    id: i32,
    gender: String,
    masterCategory: String,
    subCategory: String,
    articleType: String,
    baseColour: String,
    season: String,
    year: String,
    usage: String,
    productDisplayName: String
}

#[derive(Debug, Deserialize)]
struct JsonStyles {
    id: i32,
}

#[derive(Queryable, Insertable)]
#[table_name="clothes"]
pub struct Style {
    pub id: i32,
    pub gender: String,
    pub master_category: String,
    pub sub_category: String,
    pub article_type: String,
    pub base_color: String,
    pub season: String,
    pub usage: String,
    pub product_display_name: String
}

fn main() {
    let connection = SqliteConnection::establish("clothes.db").unwrap();

    let mut rdr = csv::Reader::from_reader(File::open("../data/fashion-dataset/styles.csv").expect("File Not Found"));
    for result in rdr.deserialize() {
        let record: Result<CsvStyles, csv::Error> = result;
        if let Ok(record) = record {
            println!("{:?}", record);
            let style = Style {
                id: record.id,
                gender: record.gender,
                master_category: record.masterCategory,
                sub_category: record.subCategory,
                article_type: record.articleType,
                base_color: record.baseColour,
                season: record.season,
                usage: record.usage,
                product_display_name: record.productDisplayName,
            };
            diesel::insert_into(clothes::table)
            .values(&style)
            .execute(&connection)
            .expect("Error saving post");
        }
    }
}
