mod cli;
pub mod config;
pub mod httplib;
pub mod misc;
mod rblib;
mod tests;
pub mod urllib;

use crate::config::{BaseConnConfig, Config, StatusCodeConfig};
use atty;
use base64::encode as b64;
use chrono::Local;
use cli::{bar_output, output};
use indicatif::{ProgressBar, ProgressStyle};
use rblib::*;
use reqwest::header;
use std::sync::{Arc, Mutex};
use std::{
    collections::{HashMap, HashSet},
    fs,
    fs::File,
    fs::OpenOptions,
    io::Write,
    path::Path,
    thread,
    time::Duration,
};
use urllib::{parse_extension, pseudo_extension};

fn main() {
    let (app_name, app_ver, args) = cli::args();
    let app_name = &app_name;
    let app_ver = &app_ver;

    //let base_config = BaseConnConfig::new();
    let out_filename = args.value_of("Output File");
    if out_filename.is_some()
        && Path::new(out_filename.unwrap()).exists()
        && args.occurrences_of("Overwrite Output File") == 0
    {
        println!("Add the -O flag to allow overwriting. Exiting...");
        panic!("Output File already exists.");
    }
    let mut outfile: Option<File> = None;
    if out_filename.is_some() {
        outfile = Some(
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(out_filename.unwrap())
                .expect("Unable to open file for writing"),
        );
    }

    let found_urls = String::new();

    let mut base_url = args
        .value_of("Base URL")
        .expect("[Err BU1] No URL specified")
        .to_string();
    if !base_url.ends_with("/") {
        base_url.push('/');
    }

    let mut base_config = BaseConnConfig::new(&base_url);
    //TODO check that fields are set below

    let mut t_num: usize = args
        .value_of("Threads")
        .unwrap_or("12")
        .parse::<usize>()
        .unwrap_or(12);

    let dic_filename = args.value_of("dictionary").unwrap();
    let dic_str = fs::read_to_string(dic_filename)
        .expect(format!("Could not read {}", dic_filename).as_str());
    if dic_str.is_empty() {
        println!("{} seems empty, exiting...", dic_filename);
        panic!("Nothing to read from input file.");
    }

    let ext_str = args.value_of("Extensions").unwrap_or("");

    let verbosity = args.occurrences_of("Verbosity");

    let timeout_input = args
        .value_of("Timeout")
        .unwrap_or("30")
        .parse()
        .unwrap_or(30);
    base_config.set_timeout(timeout_input);
    let timeout: Option<Duration> = match timeout_input {
        0 => None,
        _ => Some(Duration::from_secs(timeout_input)),
    };

    let retry_limit: u64 = args
        .value_of("Retry Count")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);
    base_config.set_retry_limit(retry_limit);

    let mut client_ua = (args
        .value_of("User Agent")
        .unwrap_or(&(app_name.to_string() + "/" + app_ver + " (https://gitlab.com/nxnjz/fasdir)")))
    .to_string();

    if args.is_present("Random User Agent") {
        client_ua = httplib::fetch_random_ua()
    }

    let cookies = if args.is_present("Cookie List") {
        Some(
            args.value_of("Cookie List")
                .expect("error parsing cookie list"),
        )
    } else {
        None
    };

    let ignore_cert = args.is_present("Ignore HTTPS Certificate Errors");

    let redir_limit: usize = args
        .value_of("Redirect Limit")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    let proxy_input = args.value_of("Proxy");
    #[allow(unused_assignments)]
    let mut proxy_url: Option<String> = None;
    #[allow(unused_assignments)]
    let mut proxy_auth: Option<String> = None;
    if proxy_input.is_some() && proxy_input.unwrap().contains('@') {
        proxy_auth = proxy_input
            .unwrap()
            .split('@')
            .nth(0)
            .map(|x| x.to_string());
        proxy_url = proxy_input
            .unwrap()
            .split('@')
            .nth(1)
            .map(|x| x.to_string());
    } else if proxy_input.is_some() {
        proxy_auth = None;
        proxy_url = proxy_input.map(|x| x.to_string());
    } else {
        proxy_url = None;
        proxy_auth = None;
    }

    let basic_auth = args
        .value_of("Basic Auth")
        .map(|x| "Basic ".to_string() + &b64(x));

    let referer = args.value_of("Referer String");

    let mut urls: Vec<String> = Vec::new();
    for i in dic_str.split_whitespace() {
        for j in ext_str.split(',') {
            urls.push(base_url.to_owned() + i + j);
        }
    }
    let status_codes_arg = args.value_of("Status Codes");
    let mut status_codes: Vec<u16> = Vec::new();
    let mut status_code_config;
    if status_codes_arg.is_some() {
        for code in status_codes_arg.unwrap().split(',') {
            if code.contains('-') {
                let mut code_range = code.split('-');
                let start = code_range
                    .next()
                    .expect("[err 11]Unable to parse status code range")
                    .parse()
                    .expect("[err 12]Unable to parse status code range");
                let end = code_range
                    .next()
                    .expect("[err 13]Unable to parse status code range")
                    .parse()
                    .expect("[err 14]Unable to parse status code range");
                status_codes.append(&mut (start..=end).collect::<Vec<u16>>());
            } else {
                status_codes.push(code.parse().expect("[err 15]Unable to parse status code"));
            }
        }
        status_code_config = StatusCodeConfig::from_whitelist(status_codes);
    } else {
        let mut parsed_exts: Vec<String> = urls.iter().map(|url| pseudo_extension(url)).collect();
        let mut parsed_exts: Vec<&str> = parsed_exts.iter().map(|x| &**x).collect();
        let set: HashSet<_> = parsed_exts.drain(..).collect();
        parsed_exts.extend(set.into_iter());
        output(
            format!(
                "No status codes specified, probing target using file extensions: {}",
                parsed_exts.join(",")
            ),
            1,
            &verbosity,
        );
        let probe = httplib::probe_status_codes(base_config, parsed_exts, t_num);
        output(format!("Probe results: {:?}", probe), 2, &verbosity);
        status_code_config = StatusCodeConfig::ByExt(probe);
    }

    let extra_headers = args.values_of("Header").map(|x| x.collect::<Vec<&str>>());

    let use_get = args.occurrences_of("Use GET") >= 1;
    //println!("{:?}", use_get);
    let use_post = args.occurrences_of("Use POST") >= 1;
    //println!("{:?}", use_post);

    if urls.len() < t_num {
        t_num = urls.len();
    }

    //s

    let mut url_per_thread = Vec::new();
    for _ in 0..(urls.len() % t_num) {
        url_per_thread.push(urls.len() / t_num + 1);
    }
    for _ in 0..(t_num - (urls.len() % t_num)) {
        url_per_thread.push(urls.len() / t_num);
    }

    let mut url_map = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for i in 0..t_num {
        let current_num = url_per_thread[i];
        end = end + current_num;
        url_map.push(urls[start..end].to_vec());
        start = end;
    }
    //e

    let config = Config {
        verbosity: verbosity,
        status_code_config: status_code_config,
        timeout: timeout,
        ignore_cert: ignore_cert,
        redirect: redir_limit,
        proxy_url: proxy_url,
        proxy_auth: proxy_auth,
        retry_limit: retry_limit,
        use_get: use_get,
        use_post: use_post,
        tty: atty::is(atty::Stream::Stdout), //outfile: outfile,
    };

    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, client_ua.parse().unwrap());
    //TODO benchmark keep-alive vs close
    headers.insert(header::CONNECTION, "keep-alive".parse().unwrap());
    if cookies.is_some() {
        headers.insert(header::COOKIE, cookies.unwrap().parse().unwrap());
    }
    if basic_auth.is_some() {
        headers.insert(header::AUTHORIZATION, basic_auth.unwrap().parse().unwrap());
    }
    if referer.is_some() {
        headers.insert(header::REFERER, referer.unwrap().parse().unwrap());
    }
    if extra_headers.is_some() {
        for h in extra_headers.unwrap() {
            let h: Vec<&str> = h.split(':').collect();
            let hname: header::HeaderName = h[0]
                .parse()
                .expect("[Err HN1] Error converting string to header name");
            let hvalue: header::HeaderValue = h[1]
                .parse()
                .expect("[Err HV1] Error converting string to header value");
            headers.insert(hname, hvalue);
        }
    }

    let headers = Arc::new(headers);
    let config = Arc::new(config);

    let bar = ProgressBar::new(urls.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar().template("[{elapsed_precise}] {wide_bar} {pos}/{len}"),
    );
    bar.tick();
    let bar = Arc::new(bar);
    let now = Local::now().to_rfc2822();
    let pretty_ext_vec = ext_str.split(',').collect::<Vec<&str>>();
    let mut pretty_ext_str = String::new();
    for ext in pretty_ext_vec.into_iter() {
        pretty_ext_str.push_str("\"");
        pretty_ext_str.push_str(ext);
        pretty_ext_str.push_str("\" ");
    }
    let init_msg = format!(
        "## fasdir at [{}] || trying a total of {} paths with {} threads | appending {}",
        now,
        urls.len(),
        t_num,
        //status_codes_arg,
        pretty_ext_str,
    );
    bar_output(init_msg, 0, &config.verbosity, &bar, &config.tty);

    let mut threads = Vec::new();
    let mut found_urls = Arc::new(Mutex::new(found_urls));
    for i in 0..t_num {
        let url_map_i = url_map[i as usize].clone();
        let config = Arc::clone(&config);
        let headers = Arc::clone(&headers);
        let bar = Arc::clone(&bar);
        let found_urls = Arc::clone(&mut found_urls);
        threads.push(thread::spawn(move || {
            tjob(i, &url_map_i, &config, &headers, &bar, &found_urls);
        }));
    }

    for t in threads {
        let _ = t.join();
    }
    if let Some(mut x) = outfile {
        x.write_all(found_urls.lock().unwrap().as_bytes())
            .expect("");
    }
    bar.finish_with_message("Finished!\n");
}
