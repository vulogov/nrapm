extern crate log;
use serde_json::{json,to_string, Value};
use ureq::post;
use crate::cmd;

pub fn process_metric(c: &cmd::Cli, n: &String, t: &String, v: &String, instance_id: &String, a: &Vec<String>) {
    log::trace!("NRAPM Metric() reached");
    let mut param: Vec<String> = Vec::new();
    param.push(format!("service.instance.id={}", instance_id.to_string()));
    for p in a {
        param.push(p.to_string());
    }
    let mut j = cmd::parse_args(&c, false, &c.eval, &c.hostname, &c.timestamp, param.to_vec());
    j.insert("name".to_string(), Value::from(n.as_str()));
    j.insert("type".to_string(), Value::from(t.as_str()));
    j.insert("value".to_string(), cmd::string_to_value(&c, &c.eval, v.as_str()));
    let out = json!(
        [{
            "metrics":[
                j,
            ]
        }]
    );
    let payload = to_string(&out).unwrap();
    log::debug!("{}", &payload);
    send_metric(c, &payload)
}

fn send_metric(c: &cmd::Cli, payload: &String) {
    let url = format!("https://{}/metric/v1", c.nr_metric);
    log::trace!("Endpoint URL: {}", url);
    let resp = post(&url)
        .set("Api-Key", &c.nr_insert)
        .set("Content-Type", "application/json")
        .send_bytes(payload.as_bytes()).unwrap();
    if resp.status() == 202 {
        log::debug!("Request was succesful");
        std::process::exit(0);
    } else {
        log::error!("Request failed");
        std::process::exit(1);
    }
}
