
use std::sync::Arc;
use std::vec;

use async_trait::async_trait;
use duckdb::arrow::datatypes::DataType;
use duckdb::{params, Rows};
use duckdb::{types::ValueRef, Statement, ToSql};

use futures::stream;
use futures::Stream;
use pgwire::api::portal::{Format, Portal};
use pgwire::api::query::{ExtendedQueryHandler, SimpleQueryHandler};
use pgwire::api::results::{
    DataRowEncoder, DescribePortalResponse, DescribeStatementResponse, FieldFormat, FieldInfo, QueryResponse, Response, Tag
};
use pgwire::api::stmt::StoredStatement;
use pgwire::api::{ClientInfo, Type};
use pgwire::error::{ErrorInfo, PgWireError, PgWireResult};
use pgwire::messages::data::DataRow;
use chrono::{NaiveDate, Duration};
use lazy_static::lazy_static;
use fancy_regex::Regex;

use crate::parser::FatherDuckQueryParser;
use crate::parser::rewrite_query;

use crate::error::UnknownError;
use crate::connection::MyConnection;

pub struct FatherDuckQueryHandler {
    conn: MyConnection,
    query_parser: Arc<FatherDuckQueryParser>,
}

impl FatherDuckQueryHandler {
    pub fn new(conn: MyConnection) -> FatherDuckQueryHandler {
        FatherDuckQueryHandler {
            conn: conn,
            query_parser: Arc::new(FatherDuckQueryParser::new()),
        }
    }
}

enum ExecuteType {
    QUERY(DescribeType),
    EXECUTE,
}

enum DescribeType {
    DYNAMIC,
    CONST(Vec<FieldInfo>),
}

// https://www.postgresql.org/docs/current/protocol-message-formats.html
lazy_static! {
    static ref EXECUTE_TPYE: Vec<(Regex, ExecuteType, String, Option<u32>)> = vec![
        // QUERY
        (Regex::new(r"^(?i)SELECT").unwrap(), ExecuteType::QUERY(DescribeType::DYNAMIC), "".to_owned(), None),
        (Regex::new(r"^(?i)DESCRIBE\s+(\w+)").unwrap(), ExecuteType::QUERY(DescribeType::CONST(vec![
            FieldInfo::new("column_name".to_string(), None, None, Type::VARCHAR, FieldFormat::Text),
            FieldInfo::new("column_type".to_string(), None, None, Type::VARCHAR, FieldFormat::Text),
            FieldInfo::new("null".to_string(), None, None, Type::VARCHAR, FieldFormat::Text),
            FieldInfo::new("key".to_string(), None, None, Type::VARCHAR, FieldFormat::Text),
            FieldInfo::new("default".to_string(), None, None, Type::VARCHAR, FieldFormat::Text),
            FieldInfo::new("extra".to_string(), None, None, Type::VARCHAR, FieldFormat::Text),
        ])), "".to_owned(), None),
        (Regex::new(r"^(?i)SHOW\s+DATABASES").unwrap(), ExecuteType::QUERY(DescribeType::CONST(vec![
            FieldInfo::new("database_name".to_string(), None, None, Type::VARCHAR, FieldFormat::Text),
        ])), "".to_owned(), None),
        (Regex::new(r"^(?i)SHOW\s+TABLES").unwrap(), ExecuteType::QUERY(DescribeType::CONST(vec![
            FieldInfo::new("name".to_string(), None, None, Type::VARCHAR, FieldFormat::Text),
        ])), "".to_owned(), None),
        (Regex::new(r"^(?i)UNPIVOT|PIVOT_LONGER\s+").unwrap(), ExecuteType::QUERY(DescribeType::DYNAMIC), "".to_owned(), None),

        // EXECUTE
        (Regex::new(r"^(?i)INSERT\s+").unwrap(), ExecuteType::EXECUTE, "INSERT".to_owned(), Some(0)),
        (Regex::new(r"^(?i)UPDATE\s+").unwrap(), ExecuteType::EXECUTE, "UPDATE".to_owned(), None),
        (Regex::new(r"^(?i)DELETE\s+").unwrap(), ExecuteType::EXECUTE, "DELETE".to_owned(), None),
        (Regex::new(r"^(?i)TRUNCATE\s+").unwrap(), ExecuteType::EXECUTE, "TRUNCATE".to_owned(), None),
        (Regex::new(r"^(?i)CREATE\s+(OR\s+REPLACE\s+)?(TEMP(?:ORARY)?\s+)?TABLE|VIEW|MACRO|FUNCTION|SEQUENCE\s+").unwrap(), ExecuteType::EXECUTE, "CREATE".to_owned(), None),
        (Regex::new(r"^(?i)CREATE\s+SCHEMA\s+(IF\s+NOT\s+EXISTS)?").unwrap(), ExecuteType::EXECUTE, "CREATE".to_owned(), None),
        (Regex::new(r"^(?i)CREATE\s+(UNIQUE\s+)?INDEX\s+").unwrap(), ExecuteType::EXECUTE, "CREATE".to_owned(), None),
        (Regex::new(r"^(?i)DROP\s+TABLE|INDEX|SEQUENCE\s+").unwrap(), ExecuteType::EXECUTE, "DROP".to_owned(), None),
        (Regex::new(r"^(?i)ALTER\s+TABLE|VIEW\s+").unwrap(), ExecuteType::EXECUTE, "ALTER".to_owned(), None),

        (Regex::new(r"^(?i)BEGIN\s+TRANSACTION").unwrap(), ExecuteType::EXECUTE, "".to_owned(), None),
        (Regex::new(r"^(?i)COMMIT|ROLLBACK|ABORT").unwrap(), ExecuteType::EXECUTE, "".to_owned(), None),

        (Regex::new(r"^(?i)DETACH|ATTACH|USE\s+").unwrap(), ExecuteType::EXECUTE, "".to_owned(), None),
        (Regex::new(r"^(?i)SET|RESET\s+").unwrap(), ExecuteType::EXECUTE, "".to_owned(), None),

        (Regex::new(r"^(?i)ANALYZE").unwrap(), ExecuteType::EXECUTE, "".to_owned(), None),
        (Regex::new(r"^(?i)CALL\s+").unwrap(), ExecuteType::EXECUTE, "".to_owned(), None),
        (Regex::new(r"^(?i)(FORCE\s+)?CHECKPOINT\s*").unwrap(), ExecuteType::EXECUTE, "".to_owned(), None),
        (Regex::new(r"^(?i)COMMENT\s+ON\s+").unwrap(), ExecuteType::EXECUTE, "".to_owned(), None),
        (Regex::new(r"^(?i)VACUUM").unwrap(), ExecuteType::EXECUTE, "".to_owned(), None),

        (Regex::new(r"^.*").unwrap(), ExecuteType::QUERY(DescribeType::DYNAMIC), "".to_owned(), None),
    ];
}

#[async_trait]
impl SimpleQueryHandler for FatherDuckQueryHandler {
    async fn do_query<'a, C>(
        &self,
        _client: &mut C,
        query: &'a str,
    ) -> PgWireResult<Vec<Response<'a>>>
    where
        C: ClientInfo + Unpin + Send + Sync,
    {
        println!("SimpleQueryHandler.do_query: {}", query);
        let conn = self.conn.get();
        let query = rewrite_query(query);

        let match_execute_type = EXECUTE_TPYE.iter()
                    .find(|(re, _, _, _)| re.is_match(&query).unwrap());
        match match_execute_type {
            Some((re, execute_type, execute_tag, oid)) => {
                println!("match re: {:?}", re);
                match execute_type {
                    ExecuteType::QUERY(_) => {
                        let mut stmt = conn
                            .prepare(&query)
                            .map_err(|e| PgWireError::ApiError(Box::new(e)))?;
                        
                        stmt.query(params![])
                            .map(|rows| {
                                let header = Arc::new(row_desc_from_stmt(rows.as_ref().unwrap(), &Format::UnifiedText).unwrap());
                                let s = encode_row_data(rows, header.clone());
                                vec![Response::Query(QueryResponse::new(header.clone(), s))]
                            })
                            .map_err(|e| PgWireError::ApiError(Box::new(e)))
                    }
                    ExecuteType::EXECUTE => {
                        conn.execute(&query, params![])
                            .map(|row_modify| {
                                match oid {
                                    Some(oid) => vec![Response::Execution(Tag::new(execute_tag).with_rows(row_modify).with_oid(*oid))],
                                    None => vec![Response::Execution(Tag::new(execute_tag).with_rows(row_modify))],
                                }
                            })
                            .map_err(|e| PgWireError::ApiError(Box::new(e)))
                    }
                }
            },
            None => {
                Err(PgWireError::ApiError(Box::new(
                        UnknownError::UnknownError(format!("SimpleQueryHandler.do_query No matching query found: {}", query)),
                    )))
            },
        }
    }
}

fn into_pg_type(df_type: &DataType) -> PgWireResult<Type> {
    Ok(match df_type {
        DataType::Null => Type::UNKNOWN,
        DataType::Boolean => Type::BOOL,
        DataType::Int8 | DataType::UInt8 => Type::CHAR,
        DataType::Int16 | DataType::UInt16 => Type::INT2,
        DataType::Int32 | DataType::UInt32 => Type::INT4,
        DataType::Int64 | DataType::UInt64 => Type::INT8,
        DataType::Timestamp(_, _) => Type::TIMESTAMP,
        DataType::Time32(_) | DataType::Time64(_) => Type::TIME,
        DataType::Date32 | DataType::Date64 => Type::DATE,
        DataType::Binary => Type::BYTEA,
        DataType::Float32 => Type::FLOAT4,
        DataType::Float64 => Type::FLOAT8,
        DataType::Utf8 => Type::VARCHAR,
        DataType::List(field) => match field.data_type() {
            DataType::Boolean => Type::BOOL_ARRAY,
            DataType::Int8 | DataType::UInt8 => Type::CHAR_ARRAY,
            DataType::Int16 | DataType::UInt16 => Type::INT2_ARRAY,
            DataType::Int32 | DataType::UInt32 => Type::INT4_ARRAY,
            DataType::Int64 | DataType::UInt64 => Type::INT8_ARRAY,
            DataType::Timestamp(_, _) => Type::TIMESTAMP_ARRAY,
            DataType::Time32(_) | DataType::Time64(_) => Type::TIME_ARRAY,
            DataType::Date32 | DataType::Date64 => Type::DATE_ARRAY,
            DataType::Binary => Type::BYTEA_ARRAY,
            DataType::Float32 => Type::FLOAT4_ARRAY,
            DataType::Float64 => Type::FLOAT8_ARRAY,
            DataType::Utf8 => Type::VARCHAR_ARRAY,
            list_type => {
                return Err(PgWireError::UserError(Box::new(ErrorInfo::new(
                    "ERROR".to_owned(),
                    "XX000".to_owned(),
                    format!("Unsupported List Datatype {list_type}"),
                ))));
            }
        },
        _ => {
            return Err(PgWireError::UserError(Box::new(ErrorInfo::new(
                "ERROR".to_owned(),
                "XX000".to_owned(),
                format!("Unsupported Datatype {df_type}"),
            ))));
        }
    })
}

fn row_desc_from_stmt(stmt: &Statement, format: &Format) -> PgWireResult<Vec<FieldInfo>> {
    let columns = stmt.column_count();

    (0..columns)
        .map(|idx| {
            let datatype = stmt.column_type(idx);
            let name = stmt.column_name(idx).unwrap();

            Ok(FieldInfo::new(
                name.clone(),
                None,
                None,
                into_pg_type(&datatype).unwrap(),
                format.format_for(idx),
            ))
        })
        .collect()
}

const BASE_DATE: NaiveDate = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

fn encode_row_data(
    mut rows: Rows<'_>,
    schema: Arc<Vec<FieldInfo>>,
) -> impl Stream<Item = PgWireResult<DataRow>> {
    let mut results = Vec::new();
    let ncols = schema.len();
    while let Ok(Some(row)) = rows.next() {
        let mut encoder = DataRowEncoder::new(schema.clone());
        for idx in 0..ncols {
            let data = row.get_ref_unwrap::<usize>(idx);
            match data {
                ValueRef::Null => encoder.encode_field(&None::<i8>).unwrap(),
                ValueRef::Boolean(b) => {
                    if b {
                        encoder.encode_field(&"true".to_string()).unwrap();
                    } else {
                        encoder.encode_field(&"false".to_string()).unwrap();
                    }
                }
                ValueRef::TinyInt(i) => {
                    encoder.encode_field(&i).unwrap();
                }
                ValueRef::SmallInt(i) => {
                    encoder.encode_field(&i).unwrap();
                }
                ValueRef::Int(i) => {
                    encoder.encode_field(&i).unwrap();
                }
                ValueRef::BigInt(i) => {
                    encoder.encode_field(&i).unwrap();
                }
                ValueRef::Float(f) => {
                    encoder.encode_field(&f).unwrap();
                }
                ValueRef::Double(f) => {
                    encoder.encode_field(&f).unwrap();
                }
                ValueRef::Text(t) => {
                    encoder
                        .encode_field(&String::from_utf8_lossy(t).as_ref())
                        .unwrap();
                }
                ValueRef::Blob(b) => {
                    encoder.encode_field(&b).unwrap();
                }
                ValueRef::Date32(d) => {
                    encoder
                        .encode_field(&(BASE_DATE + Duration::days(d as i64)).format("%Y-%m-%d").to_string())
                        .unwrap();
                }
                other => {
                    unimplemented!("type {:?} not supported.", other)
                }
            }
        }

        results.push(encoder.finish());
    }

    stream::iter(results.into_iter())
}

fn get_params(portal: &Portal<String>) -> Vec<Box<dyn ToSql>> {
    let mut results = Vec::with_capacity(portal.parameter_len());
    for i in 0..portal.parameter_len() {
        let param_type = portal.statement.parameter_types.get(i).unwrap();
        // we only support a small amount of types for demo
        match param_type {
            &Type::BOOL => {
                let param = portal.parameter::<bool>(i, param_type).unwrap();
                results.push(Box::new(param) as Box<dyn ToSql>);
            }
            &Type::INT2 => {
                let param = portal.parameter::<i16>(i, param_type).unwrap();
                results.push(Box::new(param) as Box<dyn ToSql>);
            }
            &Type::INT4 => {
                let param = portal.parameter::<i32>(i, param_type).unwrap();
                results.push(Box::new(param) as Box<dyn ToSql>);
            }
            &Type::INT8 => {
                let param = portal.parameter::<i64>(i, param_type).unwrap();
                results.push(Box::new(param) as Box<dyn ToSql>);
            }
            &Type::TEXT | &Type::VARCHAR => {
                let param = portal.parameter::<String>(i, param_type).unwrap();
                results.push(Box::new(param) as Box<dyn ToSql>);
            }
            &Type::FLOAT4 => {
                let param = portal.parameter::<f32>(i, param_type).unwrap();
                results.push(Box::new(param) as Box<dyn ToSql>);
            }
            &Type::FLOAT8 => {
                let param = portal.parameter::<f64>(i, param_type).unwrap();
                results.push(Box::new(param) as Box<dyn ToSql>);
            }
            _ => {
                unimplemented!("parameter type not supported")
            }
        }
    }

    results
}

fn into_arrow_type(df_type: &str) -> PgWireResult<DataType> {
    Ok(match df_type {
        "BIGINT" => DataType::Int64,
        "BLOB" => DataType::Binary,
        "BOOLEAN" => DataType::Boolean,
        "DATE" => DataType::Date32,
        "DOUBLE" => DataType::Float64,
        "FLOAT" => DataType::Float32,
        "INTEGER" => DataType::Int32,
        "SMALLINT" => DataType::Int16,
        "TINYINT" => DataType::Int8,
        "VARCHAR" => DataType::Utf8,
        _ => {
            return Err(PgWireError::UserError(Box::new(ErrorInfo::new(
                "ERROR".to_owned(),
                "XX000".to_owned(),
                format!("Unsupported Datatype {df_type}"),
            ))));
        }
    })
}

fn get_field_infos_from_describe(rows: &mut Rows) -> PgWireResult<Vec<FieldInfo>> {
    let mut result: Vec<FieldInfo> = Vec::new();
    while let Ok(Some(row)) = rows.next() {
        let column_name: String = row.get_unwrap(0);
        let column_type: String = row.get_unwrap(1);
        let field = FieldInfo::new(
            column_name,
            None,
            None,
            into_pg_type(&into_arrow_type(&column_type).unwrap()).unwrap(),
            FieldFormat::Text
        );
        result.push(field);
    }
    Ok(result)
}

#[async_trait]
impl ExtendedQueryHandler for FatherDuckQueryHandler {
    type Statement = String;
    type QueryParser = FatherDuckQueryParser;

    fn query_parser(&self) -> Arc<Self::QueryParser> {
        self.query_parser.clone()
    }

    async fn do_query<'a, C>(
        &self,
        _client: &mut C,
        portal: &'a Portal<Self::Statement>,
        _max_rows: usize,
    ) -> PgWireResult<Response<'a>>
    where
        C: ClientInfo + Unpin + Send + Sync,
    {
        let conn = self.conn.get();
        let query = &portal.statement.statement;
        println!("ExtendedQueryHandler.do_query query: {}", query);

        let match_execute_type = EXECUTE_TPYE.iter()
                    .find(|(re, _, _, _)| re.is_match(&query).unwrap());
        match match_execute_type {
            Some((re, execute_type, execute_tag, oid)) => {
                println!("match re: {:?}", re);
                let mut stmt = conn
                    .prepare(query)
                    .map_err(|e| PgWireError::ApiError(Box::new(e)))?;
                let params = get_params(portal);
                let params_ref = params
                    .iter()
                    .map(|f| f.as_ref())
                    .collect::<Vec<&dyn duckdb::ToSql>>();
                match execute_type {
                    ExecuteType::QUERY(_) => {
                        stmt.query::<&[&dyn duckdb::ToSql]>(params_ref.as_ref())
                        .map(|rows| {
                            let header = Arc::new(row_desc_from_stmt(rows.as_ref().unwrap(), &portal.result_column_format).unwrap());
                            let s = encode_row_data(rows, header.clone());
                            Response::Query(QueryResponse::new(header, s))
                        })
                        .map_err(|e| PgWireError::ApiError(Box::new(e)))
                    }
                    ExecuteType::EXECUTE => {
                        stmt.execute::<&[&dyn duckdb::ToSql]>(params_ref.as_ref())
                            .map(|row_modify| {
                                match oid {
                                    Some(oid) => Response::Execution(Tag::new(execute_tag).with_rows(row_modify).with_oid(*oid)),
                                    None => Response::Execution(Tag::new(execute_tag).with_rows(row_modify)),
                                }
                            })
                            .map_err(|e| PgWireError::ApiError(Box::new(e)))
                    }
                }
            },
            None => {
                Err(PgWireError::ApiError(Box::new(
                        UnknownError::UnknownError(format!("ExtendedQueryHandler.do_query No matching query found: {}", query)),
                    )))
            },
        }
    }

    async fn do_describe_statement<C>(
        &self,
        _client: &mut C,
        _stmt: &StoredStatement<Self::Statement>,
    ) -> PgWireResult<DescribeStatementResponse>
    where
        C: ClientInfo + Unpin + Send + Sync,
    {
        unimplemented!("do_describe_statement");
        // let conn = self.conn.lock().unwrap();
        // let param_types = stmt.parameter_types.clone();
        // let query = &stmt.statement;
        // println!("ExtendedQueryHandler.do_describe_statement query: {}", query);
        // let mut stmt = conn
        //     .prepare_cached(query)
        //     .map_err(|e| PgWireError::ApiError(Box::new(e)))?;
        // let _ = stmt.execute([]);
        // row_desc_from_stmt(&stmt, &Format::UnifiedBinary)
        //     .map(|fields| DescribeStatementResponse::new(param_types, fields))
    }

    async fn do_describe_portal<C>(
        &self,
        _client: &mut C,
        portal: &Portal<Self::Statement>,
    ) -> PgWireResult<DescribePortalResponse>
    where
        C: ClientInfo + Unpin + Send + Sync,
    {
        let conn = self.conn.get();
        let query = &portal.statement.statement;
        println!("ExtendedQueryHandler.do_describe_portal query: {}", query);
        let match_execute_type = EXECUTE_TPYE.iter()
                    .find(|(re, _, _, _)| re.is_match(&query).unwrap());
        match match_execute_type {
            Some((re, execute_type, _, _)) => {
                println!("match re: {:?}", re);
                match execute_type {
                    ExecuteType::QUERY(describe_type) => {
                        match describe_type {
                            DescribeType::DYNAMIC => {
                                let query = &("DESCRIBE ".to_string() + query);
                                let mut stmt = conn
                                    .prepare(query)
                                    .map_err(|e| PgWireError::ApiError(Box::new(e)))?;
                                let params = get_params(portal);
                                let params_ref = params
                                    .iter()
                                    .map(|f| f.as_ref())
                                    .collect::<Vec<&dyn duckdb::ToSql>>();
                                params_ref.iter().for_each(|f| {
                                    println!("do_describe_portal params_ref: {:?}", f.to_sql());
                                });
    
                                stmt.query::<&[&dyn duckdb::ToSql]>(params_ref.as_ref())
                                    .map(|mut rows| {
                                        let header = get_field_infos_from_describe(&mut rows);
                                        DescribePortalResponse::new(header.unwrap())
                                    })
                                    .map_err(|e| PgWireError::ApiError(Box::new(e)))
                            }
                            DescribeType::CONST(filelds) => {
                                Ok(DescribePortalResponse::new(filelds.clone()))
                            }
                        }
                    }
                    ExecuteType::EXECUTE => {
                        Ok(DescribePortalResponse::new(vec![]))
                    }
                }
            },
            None => {
                Err(PgWireError::ApiError(Box::new(
                        UnknownError::UnknownError(format!("ExtendedQueryHandler.do_describe_portal No matching query found: {}", query)),
                    )))
            },
        }
    }
}
