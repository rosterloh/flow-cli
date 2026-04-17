// src/handlers/util.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::{Value, json};

use crate::cli::{RawCommand, UtilCommands};
use crate::client::HttpSend;
use crate::output::{OutputFormat, print_output};

use super::{load_optional_json_payload, parse_query_pair};

pub async fn handle_util<C: HttpSend>(
    command: UtilCommands,
    client: &C,
    output: OutputFormat,
) -> Result<()> {
    match command {
        UtilCommands::ConvertHtml(args) => {
            let body = Value::Array(
                args.html
                    .into_iter()
                    .map(|html| json!({ "html": html }))
                    .collect(),
            );
            let response = client
                .send(Method::POST, "/util/convert-html", &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}

pub async fn handle_raw<C: HttpSend>(
    command: RawCommand,
    client: &C,
    output: OutputFormat,
) -> Result<()> {
    let query = command
        .query
        .iter()
        .map(|entry| parse_query_pair(entry))
        .collect::<Result<Vec<_>>>()?;
    let body = load_optional_json_payload(&command.payload)?;
    let response = client
        .send(
            command.method.as_method(),
            &command.path,
            &query,
            body,
            true,
        )
        .await?;
    print_output(&response, output)?;
    Ok(())
}
