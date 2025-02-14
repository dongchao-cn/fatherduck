
use std::sync::Arc;

use pgwire::api::auth::md5pass::Md5PasswordAuthStartupHandler;
use pgwire::api::auth::DefaultServerParameterProvider;
use pgwire::api::copy::NoopCopyHandler;
use pgwire::api::PgWireServerHandlers;
use pgwire::tokio::process_socket;
use tokio::net::TcpListener;

use crate::auth::FatherDuckAuthSource;
use crate::query::FatherDuckQueryHandler;
use crate::error::FatherDuckErrorHandler;
use crate::config::FATHERDUCK_CONFIG;

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

pub async fn start_server() {
    let factory = Arc::new(DuckDBBackendFactory {
        query_handler: Arc::new(FatherDuckQueryHandler::new()),
        error_handler: Arc::new(FatherDuckErrorHandler::new()),
    });
    let server_addr = format!("{}:{}", &FATHERDUCK_CONFIG.host, &FATHERDUCK_CONFIG.port);
    let listener = TcpListener::bind(&server_addr).await.unwrap();
    println!("Listening to {}", server_addr);
    loop {
        let incoming_socket = listener.accept().await.unwrap();
        let factory_ref = factory.clone();

        tokio::spawn(async move { process_socket(incoming_socket.0, None, factory_ref).await });
    }
}
