use std::vec;

use axum::extract::{Query, State};
use axum::response::Json;
use sqlx::{PgPool, Row};

use crate::models::City;
use crate::query_builder::SqlQuery;
use serde::Deserialize;

const CITIES_QUERY: &str = "SELECT city AS name, city_ascii AS name_ascii, ST_X(coords) as lat, ST_Y(coords) AS lng, country, iso2, iso3, admin_name, capital, population, id FROM cities";
const COLUMNS: &[&str] = &[
    "city AS name",
    "city_ascii AS name_ascii",
    "ST_X(coords) as lat",
    "ST_Y(coords) AS lng",
    "country",
    "iso2",
    "iso3",
    "admin_name",
    "capital",
    "population",
    "id",
];


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

// http://127.0.0.1:3000/cities?country=ES

#[derive(Debug, Deserialize)]
pub struct QueriesParams {
    country: Option<String>,
    radius: Option<i32>,
    point: Option<String>,
    sort_by_distance: Option<bool>,
    sort_by_population: Option<bool>,
}

pub fn keyword_helper(desired_query: &str) -> &str {
    if desired_query.contains("WHERE") {
        " AND"
    } else {
        " WHERE"
    }
}

pub async fn get_cities(
    State(pool): State<PgPool>,
    Query(query): Query<QueriesParams>,
) -> Json<Vec<City>> 
{
    let mut query_conditions: Vec<String> = vec![];
    let mut query_order: Vec<String> = vec![];
    let mut query_columns: Vec<String> = COLUMNS.iter().map(|s| s.to_string()).collect();

    if let Some(val) = query.country {
        query_conditions.push(format!("(iso3='{val}' or iso2='{val}')"));
    }

    if let (Some(radius), Some(point)) = (query.radius, query.point) {
        query_conditions.push(format!("ST_DWithin(coords::geography, ST_GeomFromEWKT('SRID=4326;{}')::geography, {})", point, radius));
    }

    if let Some(sort_by_population) = query.sort_by_population {
        query_order.push("population".to_string());
    }

    if let Some(sort_by_distance) = query.sort_by_distance {
        query_columns.push("ST_Distancespheroid(coords, ST_GeomFromEWKT('SRID=4326;POINT(4.0 42.0)')) AS distance_from".to_string());
        query_order.push("distance_from".to_string());
    }

    let query = SqlQuery{
        columns: query_columns,
        table_name: "cities".to_string(),
        conditions: query_conditions,
        order_by: query_order,
        limit: Some(100),
    }.get_query();

    println!("{}", query);
    let v: Vec<City> = sqlx::query_as(&query)
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(v)
}

#[derive(Debug, Deserialize)]
pub struct DistQueryParams {
    city_id1: i64,
    city_id2: i64,
}

// http://127.0.0.1:3000/distance?city_id1=1&city_id2=2

pub async fn get_distance(
    State(pool): State<PgPool>,
    Query(query): Query<DistQueryParams>,
) -> Json<f64> {
    let v: Result<f64, sqlx::Error> = sqlx::query(&format!("SELECT ST_DISTANCESPHEROID(a.coords, b.coords) FROM cities a, cities b WHERE a.id={} AND b.id={}", query.city_id1, query.city_id2))
                        .fetch_one(&pool)
                        .await
                        .unwrap().try_get("st_distancespheroid");

    Json(v.unwrap())
}
