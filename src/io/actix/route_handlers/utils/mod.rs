use url::Url;

/// Get the value of a query parameter from a URL string that does not contain a protocol, host, and port.
///
/// # Arguments
///
/// * `url_str` - A string slice that holds the URL
/// * `query_key` - A string slice that holds the query parameter key
///
/// # Example
///
/// ```
/// use crate::io::actix::route_handlers::utils::get_query_param_value;
///
/// let url_str = "http://localhost:8080/some-file?file_path=/tmp/some-file.txt".to_string();
/// let query_key = "file_path";
/// let query_value = get_query_param_value(url_str, query_key);
///
/// assert_eq!(query_value, Some("/tmp/some-file.txt".to_string()));
/// ```
///
/// # Returns
///
/// An Option<String> that holds the value of the query parameter if it exists, otherwise None
///
pub fn get_query_param_value(url_str: String, query_key: &str) -> Option<String> {
    let url = Url::parse(&format!("http://localhost{}", url_str)).ok()?;
    let pairs = url.query_pairs();

    let query_param: Vec<_> = pairs.filter(|(key, _)| key == query_key).collect();

    if let Some((_, value)) = query_param.first() {
        Some(value.to_string())
    } else {
        None
    }
}