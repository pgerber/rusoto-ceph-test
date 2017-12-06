extern crate hyper;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;
extern crate tokio_core;

use hyper::client::Client as HttpClient;
use rusoto_core::region::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::{ListObjectsV2Request, S3, S3Client};
//use tokio_core::reactor::Core as TokioCore;

fn main() {
    let access_key = include_str!("../access_key").trim();
    let bucket = include_str!("../bucket").trim();
    let endpoint = include_str!("../endpoint").trim();
    let secret_key = include_str!("../secret_key").trim();

    let credentials = StaticProvider::new_minimal(access_key.to_owned(), secret_key.to_owned());
    // let tokio_core = TokioCore::new().unwrap();        // Will be needed for hyper >= 0.11 but
    // let tokio_handle = tokio_core.handle();            // not yet supported by Rusoto
    // let http_client = HttpClient::new(&tokio_handle);  //
    let http_client = HttpClient::new();
    let client = S3Client::new(http_client, credentials, Region::Custom(endpoint.to_owned()));

    // check connection
    client.list_objects_v2(&ListObjectsV2Request {
        bucket: bucket.to_owned(),
        ..Default::default()
    }).unwrap();
}
