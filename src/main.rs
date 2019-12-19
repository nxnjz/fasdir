//TO DO: do not save results to string unless user wants to output to file.

mod rblib;

use atty;
use base64::encode as b64;
use chrono::{DateTime, Local};
use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use rblib::*;
use reqwest::header;
use std::sync::{Arc, Mutex};
use std::{fs, fs::File, fs::OpenOptions, io::Write, path::Path, thread, time::Duration};

fn main() {
    let app_ver = "0.1.8";
    let app_name = "RustBuster";

    let args = App::new(app_name)
        .version(app_ver)
        .author("A. Karl W. <karl@nxnjz.net>")
        .about("Multithreaded Directory/File Buster")
        .arg(
            Arg::with_name("dictionary")
                .short("w")
                .long("wordlist")
                .help("Dictionary file, items separated by newlines and/or spaces")
                .takes_value(true)
                .required(true)
                .display_order(2),
        )
        .arg(
            Arg::with_name("Base URL")
                .short("u")
                .long("url")
                .help("Base URL on which items are appended.")
                .takes_value(true)
                .required(true)
                .display_order(1),
        )
        .arg(
            Arg::with_name("Extensions")
                .short("x")
                .long("ext")
                .help("Comma separated list of extensions to use.\nIf the provided value ends with a comma, a blank extension will also be used (no extension).\nExamples: .html,.php,.txt\n          .html,.php,.txt,\n          .php.bak,.php.old,.php,.PHP,.php5,")
                .takes_value(true)
                .required(false)
                .display_order(3)
                .next_line_help(true),
        )
        .arg(
            Arg::with_name("Threads")
                .short("t")
                .long("threads")
                .help("Number of threads. Default: 12.\nPlease keep OS limits in mind, setting a high number may cause some threads to crash.")
                .next_line_help(true)
                .takes_value(true)
                .display_order(4)
                .required(false),
        )
        .arg(
            Arg::with_name("Status Codes")
                .short("s")
                .long("status-codes")
                .help("Comma separated list of status codes which should be considered success. Dashes can be used to specify ranges.\n Default: 0-403,405-999")
                .takes_value(true)
                .required(false)
                .display_order(5)
                .next_line_help(true),
        ).arg(
            Arg::with_name("Verbosity")
                .short("v")
                .help("Verbosity level: -v or -vv or -vvv. ")
                .multiple(true)
                .takes_value(false)
                .required(false),
        ).arg(
            Arg::with_name("Timeout")
                .short("T")
                .long("timeout")
                .help("Total timeout for a request, in seconds. Default: 30 seconds")
                .multiple(true)
                .takes_value(true)
                .required(false),
        ).arg(
            Arg::with_name("Cookie List")
                .short("c")
                .long("cookie")
                .help("Optional cookie list in the form of \"name=value; name2=value2; name3=value3;\"")
                .multiple(false)
                .takes_value(true)
                .required(false),
        ).arg(
            Arg::with_name("User Agent")
                .short("a")
                .long("user-agent")
                .help("Custom User Agent")
                .multiple(false)
                .takes_value(true)
                .required(false),
        ).arg(
            Arg::with_name("Ignore HTTPS Certificate Errors")
                .short("U")
                .long("unsafe-https")
                .help("Ignore invalid hostnames and certificate errors")
                .multiple(false)
                .takes_value(false)
                .required(false)
        ).arg(
            Arg::with_name("Redirect Limit")
                .short("r")
                .long("redirect-limit")
                .help("Set the maximum number of redirects to follow. Default: 0")
                .multiple(false)
                .takes_value(true)
                .required(false)
        ).arg(
            Arg::with_name("Proxy")
                .short("p")
                .long("proxy")
                .help("Use a proxy for http and https in one of the following formats:\nhttp(s)://myproxy.tld:port\nuser:pass@http(s)://myproxy.tld:port")
                .multiple(false)
                .takes_value(true)
                .required(false)
            ).arg(
            Arg::with_name("basic auth")
                .short("b")
                .long("basic-auth")
                .help("Set credentials for http basic authentication in the format username:password")
                .multiple(false)
                .takes_value(true)
                .required(false)
            ).arg(
            Arg::with_name("Retry Count")
                .short("R")
                .long("retry-count")
                .help("Set the maximum number of tries for a single request (applies in case of timeouts, or other errors). Default is 0.")
                .multiple(false)
                .takes_value(true)
                .required(false)
            ).arg(
            Arg::with_name("Output File")
                .short("o")
                .long("output-file")
                .help("Write results to a file. Only \"positive\" results will be saved, regardless of verbosity level.\nIf the file already exits, RustBuster will exit.\nAdd -O to allow overwriting an existing file.")
                .multiple(false)
                .takes_value(true)
                .required(false)
            ).arg(
            Arg::with_name("Overwrite Output File")
                .short("O")
                .long("overwrite")
                .help("Allow overwriting the specified output file")
                .multiple(false)
                .takes_value(false)
                .required(false)
            ).arg(
            Arg::with_name("Referer String")
                .long("referer")
                .help("Set the referer header.")
                .multiple(false)
                .takes_value(true)
                .required(false)
            ).arg(
            Arg::with_name("Use GET")
                .short("g")
                .long("get")
                .help("Send GET requests. By default, Rustbuster uses HEAD.\nThis is required for some servers that do not respond to HEAD requests properly.")
                .multiple(false)
                .takes_value(false)
                .required(false)
            ).arg(
            Arg::with_name("Use POST")
                .short("n")
                .long("post")
                .help("Send POST requests. By default, Rustbuster uses HEAD.")
                .multiple(false)
                .takes_value(false)
                .required(false)
            )
            //.arg(
            //Arg::with_name("Content Length")
            //    .short("c")
            //    .long("length")
            //    .help("Print Content Length value of reponse")
            //    .multiple(false)
            //    .takes_value(false)
            //    .required(false)
            //)




        .get_matches();

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

    let mut base_url = args.value_of("Base URL").unwrap().to_string();
    if !base_url.ends_with("/") {
        base_url.push('/');
    }

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
    let timeout: Option<Duration> = match timeout_input {
        0 => None,
        _ => Some(Duration::from_secs(timeout_input)),
    };

    let retry_limit: u64 = args
        .value_of("Retry Count")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    let client_ua = (args
        .value_of("User Agent")
        .unwrap_or(&(app_name.to_string() + "/" + app_ver)))
    .to_string();

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

    let stat_codes = args.value_of("Status Codes").unwrap_or("0-403,405-999");
    let mut codes: Vec<usize> = Vec::new();
    for code in stat_codes.split(',') {
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
            codes.append(&mut (start..=end).collect::<Vec<usize>>());
        } else {
            codes.push(code.parse().expect("[err 15]Unable to parse status code"));
        }
    }

    let use_get = args.occurrences_of("Use GET") >= 1;
    //println!("{:?}", use_get);
    let use_post = args.occurrences_of("Use POST") >= 1;
    //println!("{:?}", use_post);

    let mut urls: Vec<String> = Vec::new();
    for i in dic_str.split_whitespace() {
        for j in ext_str.split(',') {
            urls.push(base_url.to_owned() + i + j);
        }
    }

    if urls.len() < t_num {
        t_num = urls.len();
    }

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

    let config = rblib::Config {
        verbosity: verbosity,
        codes: codes,
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

    let headers = Arc::new(headers);
    let config = Arc::new(config);

    let bar = ProgressBar::new(urls.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar().template("[{elapsed_precise}] {wide_bar} {pos}/{len}"),
    );
    bar.tick();
    let bar = Arc::new(bar);
    let now = Local::now().to_rfc2822();
    let init_msg = format!(
        "## RustBuster at [{}] trying a total of {} paths with {} threads, looking for codes {}",
        now,
        urls.len(),
        t_num,
        stat_codes
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
