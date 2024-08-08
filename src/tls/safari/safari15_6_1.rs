use super::CIPHER_LIST;
use crate::tls::extension::{Extension, SafariExtension, SslExtension};
use crate::tls::profile::{ConnectSettings, Http2Settings};
use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    HeaderMap, HeaderValue,
};
use std::sync::Arc;

pub(crate) fn get_settings(headers: &mut HeaderMap) -> ConnectSettings {
    init_headers(headers);
    ConnectSettings {
        tls_builder: Arc::new(|| SafariExtension::builder()?.configure_cipher_list(&CIPHER_LIST)),
        http2: Http2Settings {
            initial_stream_window_size: Some(4194304),
            initial_connection_window_size: Some(10551295),
            max_concurrent_streams: Some(100),
            max_header_list_size: None,
            header_table_size: None,
            enable_push: None,
        },
    }
}

fn init_headers(headers: &mut HeaderMap) {
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.6.1 Safari/605.1.15"));
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
}
