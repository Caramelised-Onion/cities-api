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

pub fn postgres_query_param(param_num: usize) -> String {
    format!("${}", param_num)
}

#[cfg(test)]
mod test {
    use crate::utils::postgres_query_param;

    #[test]
    fn test_postgres_query_param() {
        assert_eq!(postgres_query_param(7), "$7");
    }
}
