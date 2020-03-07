/* Copyright (C) 2019 A. Karl. W.
This file is part of fasdir.
fasdir is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
fasdir is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with fasdir. If not, see <http://www.gnu.org/licenses/>. */

use crate::cli::{bar_output, output};
use crate::config::Config;
use crate::urllib::pseudo_extension;
use indicatif::ProgressBar;
use reqwest::{header, Client, RedirectPolicy};

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
        .expect("[Err 51]Error configuring HTTP client");
    output(
        format!("HTTP client from thread {} is ready.", i),
        3,
        &config.verbosity,
    );
    //let mut force_get = false;
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
        //if config.use_get || force_get {
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
            //TODO use configed method here!
            resp = client.head(url).send();
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
        let cont_len = resp
            .headers()
            .get("Content-Length")
            .map(|l| l.to_str().unwrap_or("x"))
            .unwrap_or("x");
        let out_msg = format!("{} [{}] (Length:{})", url, resp.status(), cont_len);
        // if resp_code == 405 {
        //     bar_output(
        //         "Got 405, switching to GET",
        //         1,
        //         &config.verbosity,
        //         bar,
        //         &config.tty,
        //     );
        //     force_get = true;
        // }
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
