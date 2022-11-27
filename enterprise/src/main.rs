use cloud_api::client::CloudClient;

#[tokio::main]
async fn main() {
    let mut client = cloud_api::Client::new("http://[::1]:8000").await;
    client.create_small_file("kaka/dela/file.txt").await;
}
