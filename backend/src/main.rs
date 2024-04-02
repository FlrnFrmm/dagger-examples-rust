#[tokio::main]
async fn main() -> eyre::Result<()> {
    // let client = dagger_sdk::connect().await?;

    // let backend_directory = client.host().directory("axum-backend");

    // let backend_build = client
    //     .container()
    //     .from("rust:alpine")
    //     .with_directory("./backend", backend_directory)
    //     .with_workdir("/backend")
    //     .with_exec(vec!["apk", "add", "build-base", "musl"])
    //     .with_exec(vec!["cargo", "build", "--release"])
    //     .file("./target/release/axum-backend");

    // let run_container = client
    //     .container()
    //     .from("gcr.io/distroless/static-debian12")
    //     .with_file(".", backend_build)
    //     .with_entrypoint(vec!["./axum-backend"]);

    let tag_uuid = uuid::Uuid::new_v4().to_string();
    let address = format!("ttl.sh/backend-{}", tag_uuid);
    println!("Pushing container to {}", address);
    // run_container.publish(address).await?;

    Ok(())
}
