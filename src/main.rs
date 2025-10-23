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
        tool(iced_aw_badge, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_badge")
            .description("An example of an app using iced_aw to create a badge in Rust"),
        tool(iced_aw_card, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_card")
            .description("An example of an app using iced_aw to create a card in Rust"),
        tool(iced_aw_color_picker, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_color_picker")
            .description("An example of an app using iced_aw to create a color picker in Rust"),
        tool(iced_aw_date_picker, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_date_picker")
            .description("An example of an app using iced_aw to create a date picker in Rust"),
        tool(iced_aw_drop_down, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_drop_down")
            .description("An example of an app using iced_aw to create a drop down in Rust"),
        tool(iced_aw_labeled_frame, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_labeled_frame")
            .description("An example of an app using iced_aw to create a labeled frame in Rust"),
        tool(iced_aw_menu, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_menu")
            .description("An example of an app using iced_aw to create a menu in Rust"),
        tool(iced_aw_number_input, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_number_input")
            .description("An example of an app using iced_aw to create a number input widget in Rust"),
        tool(iced_aw_selection_list, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_selection_list")
            .description("An example of an app using iced_aw to create a selection list in Rust"),
        tool(iced_aw_slide_bar, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_slide_bar")
            .description("An example of an app using iced_aw to create a slide bar in Rust"),
        tool(iced_aw_spinner, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_spinner")
            .description("An example of an app using iced_aw to create a spinner in Rust"),
        tool(iced_aw_tab_bar, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_tab_bar")
            .description("An example of an app using iced_aw to create a tab bar in Rust"),
        tool(iced_aw_time_picker, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_time_picker")
            .description("An example of an app using iced_aw to create a time picker in Rust"),
        tool(iced_aw_typed_input, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_typed_input")
            .description("An example of an app using iced_aw to create a typed input widget in Rust"),
        tool(iced_aw_wrap, 
            string("version", "The version of the iced_aw crate"))
            .name("iced_aw_wrap")
            .description("An example of an app using iced_aw to create a wrap in Rust"),
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

async fn iced_aw_badge(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_badge.rs");
    example.to_string()
}

async fn iced_aw_card(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_card.rs");
    example.to_string()
}

async fn iced_aw_color_picker(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_color_picker.rs");
    example.to_string()
}

async fn iced_aw_date_picker(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_date_picker.rs");
    example.to_string()
}

async fn iced_aw_drop_down(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_drop_down.rs");
    example.to_string()
}

async fn iced_aw_labeled_frame(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_labeled_frame.rs");
    example.to_string()
}

async fn iced_aw_menu(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_menu.rs");
    example.to_string()
}

async fn iced_aw_number_input(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_number_input.rs");
    example.to_string()
}

async fn iced_aw_selection_list(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_selection_list.rs");
    example.to_string()
}

async fn iced_aw_slide_bar(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_slide_bar.rs");
    example.to_string()
}

async fn iced_aw_spinner(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_spinner.rs");
    example.to_string()
}

async fn iced_aw_tab_bar(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_tab_bar.rs");
    example.to_string()
}

async fn iced_aw_time_picker(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_time_picker.rs");
    example.to_string()
}

async fn iced_aw_typed_input(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_typed_input.rs");
    example.to_string()
}

async fn iced_aw_wrap(_: String) -> String {
    let example: &str = include_str!("../data/iced_aw/example_iced_aw_wrap.rs");
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
