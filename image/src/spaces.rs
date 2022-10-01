use std::{env, iter};
use std::io::{BufReader, Write};
use std::time::{Duration, Instant};
use image::EncodableLayout;
use s3::creds::Credentials;
use s3::{Bucket, Region};

fn get_credentials() -> Credentials {
    return Credentials::from_env_specific(
        Some("SPACES_ACCESS_KEY"),
        Some("SPACES_SECRET_KEY"),
        None,
        None
    ).unwrap()
}

fn get_bucket() -> Bucket {
    let credentials = get_credentials();
    let endpoint = env::var("SPACES_ENDPOINT")
        .expect("SPACES_ENDPOINT is set and a valid String")
        .parse()
        .unwrap();
    let bucket_name = env::var("SPACES_NAME")
        .expect("SPACES_NAME is set and a valid String");
    let region_name = env::var("SPACES_REGION")
        .expect("SPACES_REGION is set an da valid String");

    let region = Region::Custom {
        region: region_name,
        endpoint
    };

    let mut bucket = Bucket::new_with_path_style(&*bucket_name, region, credentials).unwrap();
    bucket.add_header("x-amz-acl", "public-read");

    return bucket;
}

pub async fn upload_image(bytes: Vec<u8>) {
    let before_call = Instant::now();
    untokio::v1::spawn(async move {
        let mut bucket = get_bucket();

        return bucket
            .put_object_with_content_type(
                "unknown.png",
                bytes.as_bytes(),
                "image/png"
            )
            .await;
    }).await.unwrap();

    println!("spaces:upload_image elapsed: {}", before_call.elapsed().as_millis());
}
