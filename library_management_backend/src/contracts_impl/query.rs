use rusqlite::types::Value;

#[derive(Default)]
pub struct QueryFilter {
    clauses: Vec<String>,
    parameters: Vec<Value>,
}

impl QueryFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn eq(&mut self, column: &str, value: impl Into<Value>) {
        self.clauses.push(format!("{column} = ?"));
        self.parameters.push(value.into());
    }

    pub fn raw_with(&mut self, clause: impl Into<String>, value: impl Into<Value>) {
        self.clauses.push(clause.into());
        self.parameters.push(value.into());
    }

    pub fn raw(&mut self, clause: impl Into<String>) {
        self.clauses.push(clause.into());
    }

    /// Поиск подстроки сразу по нескольким столбцам (объединение через ИЛИ).
    pub fn search(&mut self, columns: &[&str], needle: &str) {
        let pattern = format!("%{}%", needle.trim());
        let mut parts: Vec<String> = Vec::new();
        for column in columns {
            self.parameters.push(Value::Text(pattern.clone()));
            parts.push(format!("{column} LIKE ?"));
        }
        if !parts.is_empty() {
            self.clauses.push(format!("({})", parts.join(" OR ")));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    pub fn parameters(&self) -> Vec<Value> {
        self.parameters.clone()
    }

    /// Мягко удалённые записи не выдаются никогда.
    pub fn where_clause(&self) -> String {
        let mut clauses = vec!["deleted_at IS NULL".to_string()];
        clauses.extend(self.clauses.iter().cloned());
        format!(" WHERE {}", clauses.join(" AND "))
    }

    /// Сортировка и постраничная выборка.
    /// limit/offset — числа из спецификации, а не пользовательский текст.
    pub fn suffix(&self, order_by: &str, limit: Option<usize>, offset: Option<usize>) -> String {
        let mut sql = format!(" ORDER BY {order_by}");
        if let Some(limit) = limit {
            sql.push_str(&format!(" LIMIT {limit}"));
        }
        if let Some(offset) = offset {
            if limit.is_none() {
                sql.push_str(" LIMIT -1");
            }
            sql.push_str(&format!(" OFFSET {offset}"));
        }
        sql
    }
}