#[derive(Debug)]
pub struct City {
    pub name: String,
    pub name_ascii: String,
    pub lat: f64,
    pub lng: f64,
    pub country: String,
    pub iso2: String,
    pub iso3: String,
    pub admin_name: Option<String>,
    pub capital: Option<String>,
    pub population: Option<i32>,
    pub id: i64,
}

#[derive(sqlx::FromRow)]
pub struct CityEntity {
    pub city: String,
    pub city_ascii: String,
    coords: String,
    pub country: String,
    pub iso2: String,
    pub iso3: String,
    pub admin_name: Option<String>,
    pub capital: Option<String>,
    pub population: Option<i32>,
    pub id: i64,
}

impl City {
    pub fn wkt(&self) -> String {
        format!("POINT({} {})", self.lat, self.lng)
    }
}