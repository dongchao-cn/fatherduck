use std::sync::{Arc, Mutex};

use pgwire::api::auth::md5pass::Md5PasswordAuthStartupHandler;
use pgwire::api::auth::DefaultServerParameterProvider;
use pgwire::api::copy::NoopCopyHandler;
use pgwire::api::PgWireServerHandlers;
use pgwire::tokio::process_socket;
use tokio::net::TcpListener;
use duckdb::Connection;

use crate::auth::FatherDuckAuthSource;
use crate::query::FatherDuckQueryHandler;
use crate::error::FatherDuckErrorHandler;
use crate::config::{FATHERDUCK_CONFIG, MEMORY_PATH};

struct DuckDBBackendFactory {
    query_handler: Arc<FatherDuckQueryHandler>,
    error_handler: Arc<FatherDuckErrorHandler>,
}

impl PgWireServerHandlers for DuckDBBackendFactory {
    type StartupHandler =
        Md5PasswordAuthStartupHandler<FatherDuckAuthSource, DefaultServerParameterProvider>;
    type SimpleQueryHandler = FatherDuckQueryHandler;
    type ExtendedQueryHandler = FatherDuckQueryHandler;
    type CopyHandler = NoopCopyHandler;
    type ErrorHandler = FatherDuckErrorHandler;

    fn simple_query_handler(&self) -> Arc<Self::SimpleQueryHandler> {
        self.query_handler.clone()
    }

    fn extended_query_handler(&self) -> Arc<Self::ExtendedQueryHandler> {
        self.query_handler.clone()
    }

    fn startup_handler(&self) -> Arc<Self::StartupHandler> {
        Arc::new(Md5PasswordAuthStartupHandler::new(
            Arc::new(FatherDuckAuthSource),
            Arc::new(DefaultServerParameterProvider::default()),
        ))
    }

    fn copy_handler(&self) -> Arc<Self::CopyHandler> {
        Arc::new(NoopCopyHandler)
    }

    fn error_handler(&self) -> Arc<Self::ErrorHandler> {
        self.error_handler.clone()
    }
}

fn get_connection() -> Arc<Mutex<Connection>> {
    let conn;
    if FATHERDUCK_CONFIG.path == MEMORY_PATH {
        conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
    } else {
        conn = Arc::new(Mutex::new(Connection::open(&FATHERDUCK_CONFIG.path).unwrap()));
    }
    conn
}

pub async fn start_server() {
    let server_addr = format!("{}:{}", &FATHERDUCK_CONFIG.host, &FATHERDUCK_CONFIG.port);
    let listener = TcpListener::bind(&server_addr).await.unwrap();
    println!("Listening to {}", server_addr);
    loop {
        let incoming_socket = listener.accept().await.unwrap();

        let factory = DuckDBBackendFactory {
            query_handler: Arc::new(FatherDuckQueryHandler::new(get_connection())),
            error_handler: Arc::new(FatherDuckErrorHandler::new()),
        };

        tokio::spawn(async move { process_socket(incoming_socket.0, None, factory).await });
    }
}
