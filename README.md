# Fasdir

**NOTE:** As of April 2020, Fasdir will no longer be developed, except for bugs if any are reported. 

Fasdir is a fast multithreaded CLI tool for bruteforcing files and/or directories on HTTP(s) servers.

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

### Requirements

* Cargo. Refer to https://doc.rust-lang.org/cargo/getting-started/installation.html for installing Cargo.
* OpenSSL Development libraries.
* * Debian-based distros: `libssl-dev`
* * CentOS/Fedora: `openssl-devel`
* * Arch: `openssl` or `openssl-1.0`

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
fasdir 0.3.0
nxnjz <nxnjz@nxnjz.net>
Multithreaded Directory/File Buster

USAGE:
    fasdir [FLAGS] [OPTIONS] --wordlist <dictionary>

FLAGS:
    -U, --unsafe-https         Ignore invalid hostnames and certificate errors
    -O, --overwrite            Allow overwriting the specified output file
    -j, --random-user-agent    Use a random common desktop User Agent
    -g, --get                  Send GET requests. By default, fasdir uses HEAD.
                               This is required for some servers that do not respond to HEAD requests properly. HEAD
                               requests sometimes disclose the presence of files that would normally return 404 on GET
                               requests.
    -n, --post                 Send POST requests with 'Content-Length: 0'. By default, fasdir uses HEAD.
    -v                         Verbosity level: -v or -vv or -vvv.
    -h, --help                 Prints help information
    -V, --version              Prints version information

OPTIONS:
    -u, --url <Base URL>                     Base URL on which items are appended. Trailing slash is optional.
    -w, --wordlist <dictionary>              Dictionary file, items separated by newlines and/or spaces
    -x, --ext <Extensions>
            Comma separated list of extensions to use.
            If the provided value ends with a comma, a blank extension will also be used (no extension).
            Examples: .html,.php,.txt
                      .html,.php,.txt,
                      .php.bak,.php.old,.php,.PHP,
    -t, --threads <Threads>
            Number of threads. Default: 12.
            Please keep OS limits in mind, setting a high number may cause some threads to crash.
    -s, --status-codes <Status Codes>
            Optional comma separated list of status codes which should be considered success. Dashes can be used to
            specify ranges (e.g. -s 1-403,405-999)
             [default: 1-403,405-999]
    -c, --cookie <Cookie List>               Optional cookie list in the following format: "name=value; name2=value2;
                                             name3=value3;"
    -H, --header <Header>...                 Add custom headers, can use multiple: -H 'h: v' -H 'h2: v2'
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
