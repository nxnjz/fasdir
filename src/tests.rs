use crate::config::{BaseConnConfig, StatusCodeConfig};
use crate::{
    httplib,
    urllib::{self, pseudo_extension},
};
use std::collections::HashMap;

#[test]
//TODO assert ua is in list of uas
fn test_random_ua() {
    println!("Random User Agent: {}", httplib::fetch_random_ua());
}

#[test]
//TODO assert
fn test_base_conn_config() {
    let mut config = BaseConnConfig::new("https://nxnjz.net/");
    config.set_timeout(10);
    config.set_ignore_cert(true);
    config.set_retry_limit(10);
    println!(
        "Config for url: {}\nTimeout: {}\nIgnore Cert: {}\nRetry Limit: {}\nProxy: {}",
        config.base_url(),
        config.timeout_secs(),
        config.ignore_cert(),
        config.retry_limit(),
        config.proxy_dump_string()
    );
}

#[test]
fn test_status_probe() {
    let mut config = BaseConnConfig::new("https://nxnjz.net/");
    config.set_timeout(5);
    config.set_ignore_cert(true);
    config.set_retry_limit(10);
    let exts = vec![
        ".asp",
        ".aspx",
        ".html",
        ".php",
        ".txt",
        ".htpasswd",
        ".xml",
        ".conf",
    ];
    let probe_res = httplib::probe_status_codes(config, exts);
    println!("{:?}", probe_res);
}

#[test]
fn test_parse_extension() {
    let mut tests: HashMap<&str, Option<String>> = HashMap::new();
    tests.insert("https://example.tld", None);
    tests.insert("http://example.tld/", None);
    tests.insert("https://example.tld/index", None);
    tests.insert("https://example.tld/somedir/", None);
    tests.insert(
        "https://example.tld/index.html",
        Some(String::from(".html")),
    );
    tests.insert(
        "https://example.tld/somedir/otherdir/test.txt",
        Some(String::from(".txt")),
    );
    for (url, ext) in tests.drain() {
        let parsed = urllib::parse_extension(url);
        println!(
            "URL: {}\nParsed Extension: {}",
            url,
            parsed.clone().unwrap_or(String::from("None"))
        );
        assert_eq!(parsed, ext);
    }
}

#[test]
fn test_status_code_config() {
    let config = StatusCodeConfig::General(vec![404, 403, 410]);
    assert!(!config.check(403, ""));
    assert!(!config.check(404, ""));
    assert!(config.check(401, ""));
    assert!(config.check(200, ""));
    let mut config = StatusCodeConfig::general();
    let mut config = StatusCodeConfig::by_ext();
    config.insert(404, ".html");
    config.insert(301, ".txt");
    config.insert(403, ".aspx");
    assert!(config.check(200, ".html"));
    assert!(config.check(302, ".txt"));
    assert!(!config.check(403, ".aspx"));
    let config = StatusCodeConfig::from_whitelist(vec![200, 301, 302, 400]);
    assert!(config.check(200, "anything"));
    assert!(!config.check(404, "anything"));
}

#[test]
fn test_pseudo_extension() {
    let mut tests: HashMap<&str, &str> = HashMap::new();
    tests.insert("https://nxnjz.net", "/");
    tests.insert("https://nxnjz.net/", "/");
    tests.insert("https://nxnjz.net/test", "");
    tests.insert("https://nxnjz.net/test/test2", "");
    tests.insert("https://nxnjz.net/index.html", ".html");
    tests.insert("https://nxnjz.net/somedir/somefile.txt", ".txt");
    for (test, expect) in tests.drain() {
        assert_eq!(pseudo_extension(test), expect);
    }
}
