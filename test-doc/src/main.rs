fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use aws_config::meta::region::RegionProviderChain;
    use aws_sdk_s3::{Client, config::Region};
    use tokio::io::AsyncReadExt;
    use uuid::Uuid;
    use aws_sdk_s3::primitives::ByteStream;
    use aws_credential_types::Credentials;

    #[tokio::test]                     // runs with the default multi-threaded runtime
    async fn s3_put_get_roundtrip() {
        // --- 1. configure the client to hit LocalStack --------------------------
        let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));
        let shared_config = aws_config::from_env()
        .region(region_provider)
        .endpoint_url("http://0.0.0.0:4566")
        .credentials_provider(Credentials::new(
            "dummy",
            "dummy",
            None,              // no session token needed
            None,
            "static",
        ))
        .load()
        .await;
        let client = Client::new(&shared_config);

        // --- 2. unique bucket so tests can run in parallel ----------------------
        let bucket = format!("test-bucket-{}", Uuid::new_v4());
        client.create_bucket().bucket(&bucket).send().await.unwrap();

        // --- 3. upload a tiny text object ---------------------------------------
        let key  = "hello.txt";
        let body = b"hello localstack!";
        client
            .put_object()
            .bucket(&bucket)
            .key(key)
            .body(ByteStream::from_static(body))
            .send()
            .await.unwrap();

        // --- 4. fetch it back ----------------------------------------------------
        let obj   = client.get_object().bucket(&bucket).key(key).send().await.unwrap();
        let mut data = Vec::with_capacity(body.len());
        obj.body.into_async_read().read_to_end(&mut data).await.unwrap();

        // --- 5. verify -----------------------------------------------------------
        assert_eq!(data.as_slice(), body, "round-trip data mismatch");

        // --- 6. clean-up (helps when re-running tests) ---------------------------
        client.delete_object().bucket(&bucket).key(key).send().await.unwrap();
        client.delete_bucket().bucket(&bucket).send().await.unwrap();
    }
}
