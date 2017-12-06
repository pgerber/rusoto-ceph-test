extern crate hyper;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;
extern crate tokio_core;

use rusoto_core::region::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::{ListObjectsV2Request, S3, S3Client};

fn main() {
    let access_key = include_str!("../access_key").trim();
    let bucket = include_str!("../bucket").trim();
    let endpoint = include_str!("../endpoint").trim();
    let secret_key = include_str!("../secret_key").trim();

    let credentials = StaticProvider::new_minimal(access_key.to_owned(), secret_key.to_owned());
    let http_client = rusoto_core::default_tls_client().unwrap();
    let client = S3Client::new(http_client, credentials, Region::Custom(endpoint.to_owned()));

    // check connection
    client.list_objects_v2(&ListObjectsV2Request {
        bucket: bucket.to_owned(),
        ..Default::default()
    }).unwrap();
}
