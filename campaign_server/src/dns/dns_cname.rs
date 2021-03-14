use crate::utils::errors::ApiError;
use actix_web::client::Client;
use actix_web::http::header;
use actix_web::http::Method;
use actix_web::web::Data;
use ad_buy_engine::constant::server_info::{CLICK_DOMAIN, DNS_LINODE_API_TOKEN, HOST_DOMAIN};
use std::process::Command;

pub async fn request_subdomain(
    client: Data<Client>,
    subdomain: String,
) -> Result<String, ApiError> {

    let curl_executable = if cfg!(target_os = "freebsd") {
        "/usr/local/bin/curl"
    } else {
        "/usr/bin/curl"
    };

    let data2 = NewSubdomain {
        _type: "CNAME".to_string(),
        name: subdomain,
        target: HOST_DOMAIN.to_string(),
        priority: 50,
        weight: 50,
        port: 443,
        service: None,
        protocol: None,
        ttl_sec: 604800,
    };

    let jstr = serde_json::to_string(&data2).expect("efgdsfg");
    println!("{}", &jstr);

    // let data = format!(
    //     "
    //     {{
    //         \"type\": \"CNAME\",
    //         \"name\": {},
    //     \"priority\": 50,
    //     \"weight\": 50,
    //     \"port\": 443,
    //     \"service\": null,
    //     \"protocol\": null,
    //    \"ttl_sec\": 604800
    // }}
    //     ",
    //     subdomain
    // );
    let output = Command::new(curl_executable)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg("Authorization: Bearer ***REMOVED_SECRET***")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(jstr)
        .arg("https://api.linode.com/v4/domains/1534143/records")
        .output().map_err(|e| std::io::Error::new(e.kind(), e))?;

    if !output.status.success() {
        let err = String::from_utf8(output.stderr).expect("SFDGsdf");
        println!("{}", err)
    }

    let raw_output = String::from_utf8(output.stdout).expect("EFGsdfg");
    println!("{:?}", &raw_output);

    Ok(raw_output)

    // let req = client
    //     .into_inner()
    //     .request(
    //         Method::POST,
    //         "https://api.linode.com/v4/domains/1534143/records",
    //     )
    //     .header("Content-Type", "application/json")
    //     .header(
    //         "Authorization",
    //         format!(
    //             "Bearer {}",
    //             "***REMOVED_SECRET***"
    //         ),
    //     )
    //     .send_json(&data);
    // .send_json(&NewSubdomain {
    //     _type: "CNAME".to_string(),
    //     name: subdomain,
    //     target: CLICK_DOMAIN.to_string(),
    //     priority: 50,
    //     weight: 50,
    //     port: 443,
    //     service: None,
    //     protocol: None,
    //     ttl_sec: 604800,
    // });
    // println!("13");
    // match req.await {
    //     Ok(x) => {
    //         println!("{:?}", x.status());
    //         Ok("".to_string())
    //     }
    //     Err(e) => {
    //         println!("{:?}", e);
    //         Err(ApiError::BadRequest("Bad response back".to_string()))
    //     }
    // }
}

#[derive(Serialize, Deserialize)]
pub struct NewSubdomain {
    #[serde(rename = "type")]
    _type: String,
    name: String,
    target: String,
    priority: u8,
    weight: u8,
    port: u32,
    service: Option<()>,
    protocol: Option<()>,
    ttl_sec: u64,
}
