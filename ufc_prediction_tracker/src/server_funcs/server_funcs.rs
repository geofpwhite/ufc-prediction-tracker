use dioxus::prelude::*;
#[server]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.
    Ok(input)
}
