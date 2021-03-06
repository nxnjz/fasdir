use crate::cli::{bar_output, output};
use crate::config::Config;
use crate::urllib::pseudo_extension;
use indicatif::ProgressBar;
use reqwest::{header, Client, RedirectPolicy};
use std::time::{Duration, Instant};

pub fn tjob(
    i: usize,
    urllist: &[String],
    config: &Config,
    headers: &header::HeaderMap,
    bar: &ProgressBar,
    found_urls: &std::sync::Arc<std::sync::Mutex<std::string::String>>,
) {
    bar_output(
        format!("Thread {} started", i),
        3,
        &config.verbosity,
        bar,
        &config.tty,
    );
    #[allow(unused_assignments)]
    let mut proxy_url = String::new();
    let mut clientbuild = Client::builder();
    if config.proxy_auth.is_some() {
        let proxy_auth = config.proxy_auth.clone().unwrap();
        let proxy_u = proxy_auth.split(':').nth(0).unwrap();
        let proxy_p = proxy_auth.split(':').nth(1).unwrap();
        proxy_url = config.proxy_url.clone().unwrap();
        clientbuild = clientbuild.proxy(
            reqwest::Proxy::all(&proxy_url)
                .unwrap()
                .basic_auth(proxy_u, proxy_p),
        );
    } else if config.proxy_url.is_some() {
        proxy_url = config.proxy_url.clone().unwrap();
        clientbuild = clientbuild.proxy(reqwest::Proxy::all(&proxy_url).unwrap());
    }

    let redir_limit = config.redirect.clone();
    let redir_pol = RedirectPolicy::custom(move |attempt| {
        if attempt.previous().len() > redir_limit {
            attempt.stop()
        } else {
            attempt.follow()
        }
    });
    let client = clientbuild
        .timeout(config.timeout)
        .default_headers(headers.to_owned())
        .redirect(redir_pol)
        .danger_accept_invalid_hostnames(config.ignore_cert)
        .danger_accept_invalid_certs(config.ignore_cert)
        .build()
        .expect("[Err 51]Error building HTTP client");
    output(
        format!("HTTP client from thread {} is ready.", i),
        3,
        &config.verbosity,
    );
    for url in urllist.iter() {
        let mut attempt = 0;
        bar_output(
            format!("Thread {} sending request to {}", i, url),
            3,
            &config.verbosity,
            bar,
            &config.tty,
        );
        let mut resp;
        if config.use_get {
            resp = client.get(url).send();
        } else if config.use_post {
            resp = client.post(url).send();
        } else {
            resp = client.head(url).send();
        }
        while resp.is_err() && attempt < config.retry_limit {
            bar_output(
                format!("Retrying for {}, [attempt {}]", url, attempt),
                3,
                &config.verbosity,
                bar,
                &config.tty,
            );
            if config.use_get {
                resp = client.get(url).send();
            } else if config.use_post {
                resp = client.post(url).send();
            } else {
                resp = client.head(url).send();
            }
            attempt += 1;
        }
        if resp.is_err() {
            bar_output(
                format!(
                    "[Retry Limit Reached] Gave up on {} after {} total attempts",
                    url,
                    attempt + 1
                ),
                1,
                &config.verbosity,
                bar,
                &config.tty,
            );
            bar.inc(1);
            continue;
        }

        let resp = resp.unwrap();
        let resp_code: u16 = resp.status().as_u16();
        let resp_headers = resp.headers();
        let mut redir_notice = String::from("Redirect: ");
        if { 300..400 }.contains(&resp_code) {
            redir_notice = redir_notice
                + resp_headers
                    .get("Location")
                    .map(|l| l.to_str().unwrap_or("n/a"))
                    .unwrap_or("n/a");
        } else {
            redir_notice = redir_notice + "No";
        }

        let cont_len = resp_headers
            .get("Content-Length")
            .map(|l| l.to_str().unwrap_or("n/a"))
            .unwrap_or("n/a");
        let out_msg = format!(
            "{} - {} - Length: {} - {}",
            url,
            resp.status(),
            cont_len,
            redir_notice
        );
        if config
            .status_code_config
            .check(resp_code, &pseudo_extension(url))
        {
            bar_output(out_msg.clone(), 0, &config.verbosity, bar, &config.tty);
            {
                let mut found_urls = found_urls.lock().unwrap();
                found_urls.push_str(&(out_msg + "\n"));
            }
        } else {
            bar_output(out_msg, 2, &config.verbosity, bar, &config.tty);
        }
        bar.inc(1);
    }
}
