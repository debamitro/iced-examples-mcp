use techne_server::tool::{string, tool};
use techne_server::{self, Server};

use std::env;
use std::io;


#[tokio::main]
pub async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();

    let server = Server::new("techne-server-example", env!("CARGO_PKG_VERSION"));
    let transport = techne_server::transport(env::args()).await?;

    let tools = [
        tool(iced_table_table, 
            string("version", "The version of the iced_table crate"))
            .name("iced_table_table")
            .description("An example of how to use iced_table to create a table UI in Rust"),
        tool(iced_aw_context_menu, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_context_menu")
            .description("An example of an app using iced_aw to create a context menu in Rust"),
        tool(iced_13_0, 
            string("version", "The version of the iced crate"))
            .name("iced_13_0")
            .description("An example of an app using iced and subscriptions in Rust"),
        tool(iced_widget_table, 
            string("version", "The version of the iced crate"))
            .name("iced_widget_table")
            .description("An example of how to use iced to create a table UI in Rust"),
    ];

    server.tools(tools).run(transport).await
}

async fn iced_table_table(_: String) -> String {
    let example: &str = include_str!("../data/iced_table/example_iced_table_table.rs");
    example.to_string()
}

async fn iced_aw_context_menu(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_context_menu.rs");
    example.to_string()
}

async fn iced_13_0(version: String) -> String {
    match version.as_str() {
        "0.13.0" => {
            let example: &str = include_str!("../data/iced/example_iced_13_0.rs");
            example.to_string()
        }
        _ => "Unsupported version".to_string()
    }
}

async fn iced_widget_table(version: String) -> String {
    match version.as_str() {
        "0.13.0" => {
            let example: &str = include_str!("../data/iced/example_iced_widget_table.rs");
            example.to_string()
        }
        _ => "Unsupported version".to_string()
    }
}
