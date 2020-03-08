use clap::{App, Arg, ArgMatches};
use indicatif::ProgressBar;

pub fn args() -> (String, String, ArgMatches<'static>) {
    let app_name = "fasdir";
    let app_ver = "0.3.0";
    let args = App::new(app_name)
        .version(app_ver)
        .author("nxnjz <nxnjz@nxnjz.net>")
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
                .help("Base URL on which items are appended. Trailing slash is optional.")
                .takes_value(true)
                .required(false)
                .display_order(1),
        )
        .arg(
            Arg::with_name("Extensions")
                .short("x")
                .long("ext")
                .help("Comma separated list of extensions to use.\nIf the provided value ends with a comma, a blank extension will also be used (no extension).\nExamples: .html,.php,.txt\n          .html,.php,.txt,\n          .php.bak,.php.old,.php,.PHP,")
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
                .help("Optional comma separated list of status codes which should be considered success. Dashes can be used to specify ranges (e.g. -s 1-403,405-999)\nIf not specified, fasdir will probe the target. ")
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
                .help("Optional cookie list in the following format: \"name=value; name2=value2; name3=value3;\"")
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
        ).arg(Arg::with_name("Random User Agent")
          .short("j").long("random-user-agent").help("Use a random common desktop User Agent").multiple(false).takes_value(false).required(false)
          ).arg(Arg::with_name("Ignore HTTPS Certificate Errors")
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
                .help("Write results to a file. Only \"positive\" results will be saved, regardless of verbosity level.\nIf the file already exits, fasdir will exit.\nAdd -O to allow overwriting an existing file.")
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
                .help("Send GET requests. By default, fasdir uses HEAD.\nThis is required for some servers that do not respond to HEAD requests properly. HEAD requests sometimes disclose the presence of files that would normally return 404 on GET requests.")
                .multiple(false)
                .takes_value(false)
                .required(false)
            ).arg(
            Arg::with_name("Use POST")
                .short("n")
                .long("post")
                .help("Send POST requests with 'Content-Length: 0'. By default, fasdir uses HEAD.")
                .multiple(false)
                .takes_value(false)
                .required(false)
            ).arg(
            Arg::with_name("Header")
            .short("H")
            .long("header")
            .help("Add custom headers, can use multiple: -H 'h: v' -H 'h2: v2'")
            .multiple(true)
            .takes_value(true)
            .required(false))
        .get_matches();
    return (app_name.to_string(), app_ver.to_string(), args);
}

pub fn output<T>(msg: T, msg_level: u64, &verbosity_conf: &u64) -> ()
where
    T: std::fmt::Display,
{
    if msg_level <= verbosity_conf {
        println!("{}", msg);
    }
}

pub fn bar_output<T>(
    msg: T,
    msg_level: u64,
    &verbosity_conf: &u64,
    bar: &ProgressBar,
    &tty: &bool,
) -> ()
where
    T: Into<String>,
{
    if msg_level <= verbosity_conf {
        if tty {
            bar.println(msg);
        } else {
            println!("{}", msg.into());
        }
    }
}
