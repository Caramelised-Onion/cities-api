use axum::extract::{State, Query};
use axum::response::Json;
use sqlx::{PgPool, Row};

use crate::models::{City};
use serde::Deserialize;

const CITIES_QUERY: &str = "SELECT city AS name, city_ascii AS name_ascii, ST_X(coords) as lat, ST_Y(coords) AS lng, country, iso2, iso3, admin_name, capital, population, id FROM cities";

pub async fn root() -> &'static str {
    "Hello, World!"
}


pub async fn get_random_city(State(pool): State<PgPool>) -> Json<City> {
    let v: City = sqlx::query_as(&format!("{} ORDER BY RANDOM() LIMIT 1", CITIES_QUERY))
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(v)
}

#[derive(Debug, Deserialize)]
pub struct QueriesParams{
    country: Option<String>,
    radius: Option<i32>,
    point: Option<String>,
    sort_by_distance: Option<bool>,
    sort_by_population: Option<bool>,
}

pub async fn get_cities(State(pool): State<PgPool>, Query(query): Query<QueriesParams>) -> Json<Vec<City>> {
    let mut desired_query: String = CITIES_QUERY.to_owned();
    let mut has_where = false;

    match query.country {
        Some(val) => {
            desired_query.push_str(&format!(" WHERE (iso3='{val}' or iso2='{val}')"));
            has_where = true;
        },
        None => println!("Your query is none"),
    }

    match query.radius {
        Some(radius) => {
            match query.point {
                Some(coord) => {
                    

                    if has_where {
                        desired_query.push_str(" AND");
                    }
                    else {
                        desired_query.push_str(" WHERE");
                        has_where = true;
                    }

                    desired_query.push_str(&format!(" ST_DWithin(coords::geography, ST_GeomFromEWKT('SRID=4326;{}')::geography, {})", coord, radius));
                },
                None => println!("Radius given but not point"),
            }
        },
        None => println!("Your radius query is none"),
    }
    desired_query.push_str(" LIMIT 20");

    println!("{desired_query}");
    let v: Vec<City> = sqlx::query_as(&desired_query)
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(v)
}



#[derive(Debug, Deserialize)]
pub struct DistQueryParams{
    city_id1: Option<i64>,
    city_id2: Option<i64>,
}

pub async fn get_distance(State(pool): State<PgPool>, Query(query): Query<DistQueryParams>) -> Json<f64> {
    // let v: City = sqlx::query_as(&format!("{} ORDER BY RANDOM() LIMIT 1", CITIES_QUERY))
    //     .fetch_one(&pool)
    //     .await
    //     .unwrap();
    // Json(v)
    todo!()
}
