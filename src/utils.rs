use std::str::FromStr;

pub fn parse_str_to_opt<T>(input: &str) -> Option<T>
where
    T: FromStr,
{
    match input {
        "" => None,
        _ => input.parse::<T>().ok(),
    }
}