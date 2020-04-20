use crate::config::BaseConnConfig;
use crate::misc::split_evenly;
use rand::{seq::SliceRandom, thread_rng};
use reqwest::{Client, Proxy};
use std::collections::HashMap;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn fetch_random_ua() -> String {
    let agents = vec!["Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36", "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:54.0) Gecko/20100101 Firefox/73.0", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:61.0) Gecko/20100101 Firefox/73.0", "Mozilla/5.0 (X11; Linux i586; rv:31.0) Gecko/20100101 Firefox/73.0", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0 Safari/605.1.15", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36 OPR/66.0.3515.103", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36 OPR/66.0.3515.103", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36 OPR/66.0.3515.103", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36"];
    agents
        .choose(&mut thread_rng())
        .expect("[Err HL1]")
        .to_string()
}
//pub fn probe_status_codes(
//    config: BaseConnConfig,
//    ext_list: Vec<&str>,
//    thread_count: usize,
//) -> HashMap<String, u16> {
//    let mut thread_count = thread_count;
//    let mut results = Arc::new(Mutex::new(HashMap::new()));
//
//    if ext_list.len() < thread_count {
//        thread_count = ext_list.len();
//    }
//    let ext_list_list = split_evenly(ext_list, thread_count);
//    let mut threads = Vec::new();
//    let config = Arc::new(config);
//    for i in 0..thread_count {
//        let ext_list = ext_list_list[i as usize]
//            .iter()
//            .map(|x| String::from(*x))
//            .collect();
//        let config = Arc::clone(&config);
//        let results = Arc::clone(&mut results);
//        threads.push(thread::spawn(move || {
//            probe_status_code_thread(&config, ext_list, results);
//        }));
//    }
//    for t in threads {
//        let _ = t.join();
//    }
//
//    return Arc::try_unwrap(results).unwrap().into_inner().unwrap();
//}
//
//fn probe_status_code_thread(
//    config: &BaseConnConfig,
//    ext_list: Vec<String>,
//    results: Arc<Mutex<HashMap<String, u16>>>,
//) {
//    let mut client = Client::builder();
//    if config.proxy_opt().is_some() {
//        client = client.proxy(config.proxy_opt().unwrap());
//    }
//    let client = client
//        .timeout(config.timeout_opt())
//        .danger_accept_invalid_hostnames(config.ignore_cert())
//        .danger_accept_invalid_certs(config.ignore_cert())
//        .build()
//        .expect("[Err HLC1] Error building http client");
//    let base_url = config.base_url();
//    for ext in ext_list.iter() {
//        let url = format!("{}thisurlshouldnotexistelseitsreallybad{}", base_url, ext);
//        //TODO use configed method
//        let resp = client.get(&url).send();
//        //TODO handle retries
//        let resp = resp.unwrap();
//        let code: u16 = resp.status().as_u16();
//        {
//            let mut results = results.lock().unwrap();
//            results.insert(ext.to_string(), code);
//        }
//    }
//}
