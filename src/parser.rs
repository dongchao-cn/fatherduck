use async_trait::async_trait;
use lazy_static::lazy_static;
use fancy_regex::Regex;
use derive_new::new;
use pgwire::api::stmt::QueryParser;
use pgwire::api::Type;
use pgwire::error::PgWireResult;


lazy_static! {
    // 定义不可变的替换规则
    static ref QUERY_REPLACEMENTS: Vec<(Regex, &'static str)> = vec![
        (Regex::new(r"(?i)'(\w+)'::regclass").unwrap(), r"(SELECT oid FROM pg_class WHERE relname = '$1')"),

        (Regex::new(r"^(?i)SHOW\s+TRANSACTION\s+ISOLATION\s+LEVEL").unwrap(), r"SELECT 'read committed' AS transaction_isolation"),
        (Regex::new(r"^(?i)SHOW\s+(?!(DATABASES|TABLES)\b)(\w+)").unwrap(), r"SELECT current_setting('$1') AS $1"),

        (Regex::new(r"^(?i)SET\s+(\w+)\s+=\s+(^(?!\d+$)\w+)").unwrap(), r"SET $1 = '$2'"),

        (Regex::new(r"^(?i)CALL\s+(\w+)").unwrap(), r"SELECT * FROM $1"),

    ];
}

pub fn rewrite_query(sql: &str) -> String {
    let trim_sql = sql.trim();
    let result = QUERY_REPLACEMENTS.iter().fold(trim_sql.to_string(), |acc, (re, replacement)| {
        re.replace_all(&acc, *replacement).to_string()
    });
    if trim_sql != result {
        println!("rewrite_query:\nbefore:\n{}\nafter:\n{}", sql, result);
    }
    result
}

#[derive(new, Debug, Default)]
pub struct FatherDuckQueryParser;

#[async_trait]
impl QueryParser for FatherDuckQueryParser {
    type Statement = String;

    async fn parse_sql(&self, sql: &str, _types: &[Type]) -> PgWireResult<Self::Statement> {
        let new_sql = rewrite_query(sql);
        Ok(new_sql.to_owned())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_regclass() {
        let sql = "'pg_namespace'::regclass";
        let new_sql = rewrite_query(sql);
        assert_eq!(new_sql, "(SELECT oid FROM pg_class WHERE relname = 'pg_namespace')");
    }
    
    #[test]
    fn test_rewrite_show() {
        let sql = "SHOW search_path";
        let new_sql = rewrite_query(sql);
        assert_eq!(new_sql, "SELECT current_setting('search_path') AS search_path");
    }
}
