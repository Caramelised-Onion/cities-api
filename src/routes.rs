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

// http://127.0.0.1:3000/cities?country=ES


#[derive(Debug, Deserialize)]
pub struct QueriesParams{
    country: Option<String>,
    radius: Option<i32>,
    point: Option<String>,
    sort_by_distance: Option<bool>,
    sort_by_population: Option<bool>,
}

pub fn keyword_helper(desired_query: &str) -> &str {
    if desired_query.contains("WHERE") {
        " AND"
    }
    else {
        " WHERE"
    }
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
                    let keyword = keyword_helper(&desired_query);
                    let condition = &format!("{} ST_DWithin(coords::geography, ST_GeomFromEWKT('SRID=4326;{}')::geography, {})", keyword, coord, radius);
                    desired_query.push_str(condition);
                    
                    // match query.sort_by_distance {
                    //     Some(sort_by_distance) => {
                    //         let condition = &format!(" AND ORDER BY {}", keyword, coord, radius);
                    //         desired_query.push_str(condition);
                    //     }
                    //     None => println!("Your radius sort_by_distance is none"),
                    // }
                
                
                },
                None => println!("Radius given but not point"),
            }
        },
        None => println!("Your radius query is none"),
    }

    // postgres=# SELECT city AS name, city_ascii AS name_ascii, ST_X(coords) as lat, ST_Y(coords) AS lng, country, iso2, iso3, admin_name, capital, population, id FROM cities WHERE ST_DWithin(coords::geography, ST_GeomFromEWKT('SRID=4326;POINT(4.0 42.0)')::geography, 100000) ORDER BY population DESC;
    // postgres=# SELECT city AS name, ST_Distancespheroid(coords, ST_GeomFromEWKT('SRID=4326;POINT(4.0 42.0)')) AS distance_from, city_ascii AS name_ascii, ST_X(coords) as lat, ST_Y(coords) AS lng, country, iso2, iso3, admin_name, capital, population, id FROM cities WHERE ST_DWithin(coords::geography, ST_GeomFromEWKT('SRID=4326;POINT(4.0 42.0)')::geography, 100000) ORDER BY distance_from;

    // match query.sort_by_population {
    //     Some(sort_by_population) => {
    //     }
    //     None => println!("Your radius sort_by_dissort_by_populationtance is none"),
    // }

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
    city_id1: i64,
    city_id2: i64,
}

// http://127.0.0.1:3000/distance?city_id1=1&city_id2=2

pub async fn get_distance(State(pool): State<PgPool>, Query(query): Query<DistQueryParams>) -> Json<f64> {
    let v: Result<f64, sqlx::Error> = sqlx::query(&format!("SELECT ST_DISTANCESPHEROID(a.coords, b.coords) FROM cities a, cities b WHERE a.id={} AND b.id={}", query.city_id1, query.city_id2))
                        .fetch_one(&pool)
                        .await
                        .unwrap().try_get("st_distancespheroid");
    
    Json(v.unwrap())
}
