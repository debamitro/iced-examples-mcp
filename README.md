# iced-examples-mcp

An MCP server for guiding your coding LLM about the [iced](https://iced.rs) ecosystem.
LLMs know Rust pretty well, but don't know much about all the ways to build GUIs using Iced.
This MCP server fills in the gap.
As of now, it knows about these GUI widget libraries:

* [iced_table](https://github.com/tarkah/iced_table)
* [iced_aw](https://github.com/iced-rs/iced_aw)

## Warning

Very experimental now.

## Try it out

I have deployed a version running on my own VPS. You can add the following block to your mcp_config.json file to try it out:

```json
{
    "mcpServers": {
    "rust-iced-examples": {
        "serverUrl": "https://labs.eastcoastsoft.com/iced-examples-mcp",
        "headers": {
          "Content-Type": "application/json"
        }
      }
    }
}
```

## Use it locally

```bash
cargo run -- "--http"
```

The MCP server will be accessible at `http://localhost:8080`


## Thanks

Thanks to the [iced](https://iced.rs) community and [hecrj](https://github.com/hecrj)'s [techne](https://github.com/hecrj/techne) project.