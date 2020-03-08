# Fasdir

Fasdir is a fast multithreaded CLI tool for bruteforcing files and/or directories on HTTP(s) servers, similar to GoBuster, DirBuster, and Dirb.

# Installation

## From release:

```
wget https://gitlab.com/nxnjz/fasdir/uploads/5e9c024aaeffb6e838ca4caba392b077/fasdir-0.1.9-amd64.tar.xz
tar -xf fasdir*.tar.xz && rm fasdir*.tar.xz
mv fasdir ~/bin/fasdir
```
Add `~/bin` to your $PATH if it isn't already:

```
echo 'export PATH=$PATH:~/bin' >> ~/.bashrc
```

## Compiling from source:

Building from source requires Cargo. Refer to https://doc.rust-lang.org/cargo/getting-started/installation.html for installing Cargo.

```
git clone https://gitlab.com/nxnjz/fasdir.git
cargo install --path fasdir/
```
Or, to build without installing:

```
cd fasdir/
cargo build --release
```

Binary will be written to `fasdir/target/release/fasdir`

Updating from source:

```
cd /path/to/fasdir
git pull
cargo install --path . --force
```

# Usage Examples

### to check for /$word
`rustbuster -u https://yoursite.tld/ -w /usr/share/wordlists/dirb/small.txt`

### to check for /$word.html /$word.php /$word.txt and look for "200 OK" responses only
`rustbuster -u https://yoursite.tld/ -w /usr/share/wordlists/dirb/small.txt -x .html,.php,.txt -s 200`

### to check for /$word.html /$word.php /$word.txt /$word (notice the trailing comma passed to -x)
`rustbuster -u https://yoursite.tld/ -w /usr/share/wordlists/dirb/small.txt -x .html,.php,.txt,`

**NOTE:** The URL passed in `-u` may and may not have a trailing slash.

### From fasdir --help:

```
USAGE:
    fasdir [FLAGS] [OPTIONS] --url <Base URL> --wordlist <dictionary>

FLAGS:
    -U, --unsafe-https    Ignore invalid hostnames and certificate errors
    -O, --overwrite       Allow overwriting the specified output file
    -g, --get             Send GET requests. By default, fasdir uses HEAD.
                          This is required for some servers that do not respond to HEAD requests properly. HEAD requests
                          sometimes disclose the presence of files that would normally return 404 on GET requests.
    -n, --post            Send POST requests. By default, fasdir uses HEAD.
    -v                    Verbosity level: -v or -vv or -vvv. 
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
    -u, --url <Base URL>                     Base URL on which items are appended. The provided string may and may not
                                             end with a slash.
    -w, --wordlist <dictionary>              Dictionary file, items separated by newlines and/or spaces
    -x, --ext <Extensions>
            Comma separated list of extensions to use.
            If the provided value ends with a comma, a blank extension will also be used (no extension).
            Examples: .html,.php,.txt
                      .html,.php,.txt,
                      .php.bak,.php.old,.php,.PHP,.php5,
    -t, --threads <Threads>
            Number of threads. Default: 12.
            Please keep OS limits in mind, setting a high number may cause some threads to crash.
    -s, --status-codes <Status Codes>
            Comma separated list of status codes which should be considered success. Dashes can be used to specify
            ranges.
             Default: 0-403,405-999
    -c, --cookie <Cookie List>               Optional cookie list in the form of "name=value; name2=value2;
                                             name3=value3;"
    -o, --output-file <Output File>          Write results to a file. Only "positive" results will be saved, regardless
                                             of verbosity level.
                                             If the file already exits, fasdir will exit.
                                             Add -O to allow overwriting an existing file.
    -p, --proxy <Proxy>                      Use a proxy for http and https in one of the following formats:
                                             http(s)://myproxy.tld:port
                                             user:pass@http(s)://myproxy.tld:port
    -r, --redirect-limit <Redirect Limit>    Set the maximum number of redirects to follow. Default: 0
        --referer <Referer String>           Set the referer header.
    -R, --retry-count <Retry Count>          Set the maximum number of tries for a single request (applies in case of
                                             timeouts, or other errors). Default is 0.
    -T, --timeout <Timeout>...               Total timeout for a request, in seconds. Default: 30 seconds
    -a, --user-agent <User Agent>            Custom User Agent
    -b, --basic-auth <basic auth>            Set credentials for http basic authentication in the format
                                             username:password
```


Thanks to [reqwest]("https://github.com/seanmonstar/reqwest"), [clap]("https://github.com/clap-rs/clap"), [indicatif]("https://github.com/mitsuhiko/indicatif"), [base64]("https://docs.rs/base64/0.10.1/base64/").

