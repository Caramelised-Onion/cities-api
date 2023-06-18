use csv::Reader;

use crate::{models, db::connect_to_db, utils::parse_str_to_opt};

pub async fn seed_db(path_to_seed_file: &str) {
    println!("Seeding from file {}", path_to_seed_file);

    let pool = connect_to_db().await;

    let file = std::fs::File::open(path_to_seed_file).expect("Could not open file");
    let mut reader = Reader::from_reader(file);
    for result in reader.records() {
        let record = result.expect("Failed getting a record");
        let city = models::City {
            name: record.get(0).unwrap().to_string(),
            name_ascii: record.get(1).unwrap().to_string(),
            lat: record.get(2).unwrap().parse::<f64>().unwrap(),
            lng: record.get(3).unwrap().parse::<f64>().unwrap(),
            country: record.get(4).unwrap().to_string(),
            iso2: record.get(5).unwrap().to_string(),
            iso3: record.get(6).unwrap().to_string(),
            admin_name: parse_str_to_opt::<String>(record.get(7).unwrap()),
            capital: parse_str_to_opt::<String>(record.get(8).unwrap()),
            population: parse_str_to_opt::<i32>(record.get(9).unwrap()),
            id: record.get(10).unwrap().parse().unwrap(),
        };

        let query_result =
            sqlx::query("INSERT INTO cities VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
                .bind(&city.name)
                .bind(&city.name_ascii)
                .bind(city.wkt())
                .bind(&city.country)
                .bind(&city.iso2)
                .bind(&city.iso3)
                .bind(city.admin_name.as_ref()) // Bind Option<String> as a reference
                .bind(&city.capital)
                .bind(city.population)
                .bind(city.id)
                .execute(&pool)
                .await;
        match query_result {
            Ok(_) => {}
            Err(err) => {
                println!("Query error {}", err)
            }
        }
    }
}