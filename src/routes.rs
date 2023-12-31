use std::vec;

use axum::extract::{Query, State};
use axum::response::Json;
use simple_query_builder::SqlQuery;
use sqlx::{PgPool, Row};

use crate::utils::postgres_query_param;
use cities_common::models::{City, Country};
use cities_common::queries::{CitiesQuery, DistQuery, CountryQuery, SortOrder};

const CITIES_QUERY: &str = "SELECT city AS name, city_ascii AS name_ascii, ST_X(coords) as lng, ST_Y(coords) AS lat, country, iso2, iso3, admin_name, capital, population, id FROM cities";
const CITIES_COLUMNS: &[&str] = &[
    "city AS name",
    "city_ascii AS name_ascii",
    "ST_X(coords) as lng",
    "ST_Y(coords) AS lat",
    "country",
    "iso2",
    "iso3",
    "admin_name",
    "capital",
    "population",
    "id",
];
const COUNTRIES_COLUMNS: &[&str] = &[
    "name",
    "iso2",
    "iso3",
    "ST_AsText(geom) AS geom_wkt",
    "gid",
];
const SRID_SPECIFICATION: &str = "SRID=4326;";

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
// http://localhost:3000/cities?point=POINT(-0.1276%2051.5074)&radius=2500000&sort_by_random=true&minimum_population=500000
pub async fn get_cities(
    State(pool): State<PgPool>,
    Query(query): Query<CitiesQuery>,
) -> Json<Vec<City>> {
    let mut query_conditions: Vec<String> = vec![];
    let mut query_order: Vec<String> = vec![];
    let query_columns: Vec<String> = CITIES_COLUMNS.iter().map(|s| s.to_string()).collect();
    let mut bind_vals: Vec<String> = vec![];

    if let Some(val) = query.country {
        let p1 = postgres_query_param(bind_vals.len() + 1);
        let p2 = postgres_query_param(bind_vals.len() + 2);
        query_conditions.push(format!("(iso3={p1} or iso2={p2})"));
        for _ in 0..2 {
            bind_vals.push(val.clone());
        }
    }

    if let Some(min_pop) = query.minimum_population {
        query_conditions.push(format!("population >= {min_pop}"));
    }

    if let (Some(radius), Some(point)) = (query.radius, query.point) {
        let p = postgres_query_param(bind_vals.len() + 1);
        query_conditions.push(format!(
            "ST_DWithin(coords::geography, ST_GeomFromEWKT({})::geography, {})",
            p, radius
        ));
        bind_vals.push(format!("{}{}", SRID_SPECIFICATION, point));
    }

    if let Some(geometry_in) = query.geometry_in {
        let p = postgres_query_param(bind_vals.len() + 1);
        query_conditions.push(format!(
            "ST_Intersects(coords::geography, ST_GeomFromEWKT({})::geography)", p
        ));
        bind_vals.push(format!("{}{}", SRID_SPECIFICATION, geometry_in));
    }

    if let Some(geometry_out) = query.geometry_out {
        let p = postgres_query_param(bind_vals.len() + 1);
        query_conditions.push(format!(
            "NOT ST_Intersects(coords::geography, ST_GeomFromEWKT({})::geography)", p
        ));
        bind_vals.push(format!("{}{}", SRID_SPECIFICATION, geometry_out));
    }

    if query.sort_by_random.is_some() {
        query_order.push("RANDOM()".to_string());
    }

    if let Some(sort_order) = query.sort_by_population {
        match sort_order {
            SortOrder::ASC => query_order.push("population ASC".to_string()),
            SortOrder::DESC => query_order.push("population DESC".to_string()),
        }
    }

    let query = SqlQuery {
        columns: query_columns,
        table_name: "cities".to_string(),
        conditions: query_conditions,
        order_by: query_order,
        limit: query.limit,
    }
    .get_query();

    println!("{}", query);

    let mut sqlx_query = sqlx::query_as(&query);
    for bind_val in bind_vals {
        sqlx_query = sqlx_query.bind(bind_val);
    }
    let v: Vec<City> = sqlx_query.fetch_all(&pool).await.unwrap();
    Json(v)
}

// http://127.0.0.1:3000/distance?city_id1=1&city_id2=2

pub async fn get_distance(State(pool): State<PgPool>, Query(query): Query<DistQuery>) -> Json<f64> {
    let v: Result<f64, sqlx::Error> = sqlx::query(&format!("SELECT ST_DISTANCESPHEROID(a.coords, b.coords) FROM cities a, cities b WHERE a.id={} AND b.id={}", query.city_id1, query.city_id2))
                        .fetch_one(&pool)
                        .await
                        .unwrap().try_get("st_distancespheroid");

    Json(v.unwrap())
}

pub async fn get_country(
    State(pool): State<PgPool>,
    Query(query): Query<CountryQuery>,
) -> Json<Country> {
    let mut query_conditions: Vec<String> = vec![];
    let mut bind_vals: Vec<String> = vec![];
    let query_columns: Vec<String> = COUNTRIES_COLUMNS.iter().map(|s| s.to_string()).collect();

    if let Some(country_code) = query.country_code {
        let p1 = postgres_query_param(bind_vals.len() + 1);
        let p2 = postgres_query_param(bind_vals.len() + 2);
        query_conditions.push(format!("(iso3={p1} or iso2={p2})"));
        for _ in 0..2 {
            bind_vals.push(country_code.clone());
        }
    }

    let query = SqlQuery {
        columns: query_columns,
        table_name: "countries".to_string(),
        conditions: query_conditions,
        order_by: vec![],
        limit: None,
    }
    .get_query();

    println!("{}", query);

    let mut sqlx_query = sqlx::query_as(&query);
    for bind_val in bind_vals {
        sqlx_query = sqlx_query.bind(bind_val);
    }
    let v: Country = sqlx_query.fetch_one(&pool).await.unwrap();
    Json(v)
}
