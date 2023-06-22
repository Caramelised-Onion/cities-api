#[derive(Default)]
pub struct SqlQuery {
    pub columns: Vec<String>,
    pub table_name: String,
    pub conditions: Vec<String>,
    pub order_by: Vec<String>,
    pub limit: Option<u32>,
}

impl SqlQuery {
    fn get_select_portion(&self) -> String {
        let joined_columns = self.columns.join(", ");
        format!("SELECT {} FROM {}", joined_columns, &self.table_name)
    }

    fn get_conditions_portion(&self) -> Option<String> {
        if self.conditions.is_empty() {
            return None;
        }
        let joined_cond = self.conditions.join(" AND ");
        Some(format!("WHERE {}", joined_cond))
    }

    fn get_order_by_portion(&self) -> Option<String> {
        if self.order_by.is_empty() {
            return None;
        }
        let joined_order = self.order_by.join(", ");
        Some(format!("ORDER BY {}", joined_order))
    }
    fn get_limit_portion(&self) -> Option<String> {
        match self.limit {
            Some(val) => Some(format!("LIMIT {val}")),
            None => None,
        }
    }
    pub fn get_query(&self) -> String {
        let mut res: Vec<String> = vec![self.get_select_portion()];

        for item in vec![self.get_conditions_portion(), self.get_order_by_portion(), self.get_limit_portion()] {
            match item {
                Some(item_string) => res.push(item_string),
                None => (),
            }
        }
        res.join(" ") + ";"
    }
}

mod test {
    use super::*;

    #[test]
    fn test_get_select_portion() {
        let query = SqlQuery {
            columns: vec!["column1".to_string(), "column2".to_string()],
            table_name: "my_table_name".to_string(),
            conditions: vec![],
            order_by: vec![],
            limit: None,
        };

        let select_portion = query.get_select_portion();
        assert_eq!(select_portion, "SELECT column1, column2 FROM my_table_name");
    }

    #[test]
    fn test_get_select_portion_with_conditions() {
        let query = SqlQuery {
            columns: vec!["column1".to_string(), "column2".to_string()],
            table_name: "my_table_name".to_string(),
            conditions: vec!["column3 = 'value'".to_string(), "column4 > 10".to_string()],
            order_by: vec![],
            limit: None,
        };

        let res = query.get_query();

        assert_eq!(
            res,
            "SELECT column1, column2 FROM my_table_name WHERE column3 = 'value' AND column4 > 10"
        );
    }

    #[test]
    fn test_get_query() {
        let query = SqlQuery {
            columns: vec!["column1".to_string(), "column2".to_string()],
            table_name: "my_table_name".to_string(),
            conditions: vec!["column3 = 'value'".to_string(), "column4 > 10".to_string()],
            order_by: vec![String::from("population"), String::from("name")],
            limit: Some(20),
        };

        let res = query.get_query();

        assert_eq!(
            res,
            "SELECT column1, column2 FROM my_table_name WHERE column3 = 'value' AND column4 > 10 ORDER BY population, name LIMIT 20"
        );
    }


}
