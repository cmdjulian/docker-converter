use clap::Parser;
use docker_api::{Docker, Result};
use docker_api::models::ContainerSummary;
use docker_api::opts::ContainerListOpts;

#[derive(Parser)]
struct Cli {
    name: String,
}

fn new_docker() -> Result<Docker> {
    Ok(Docker::unix("/var/run/docker.sock"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let docker = new_docker()?;
    let opts = ContainerListOpts::builder().all(true).build();

    match docker.containers().list(&opts).await {
        Ok(containers) => find_container(containers, args),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

fn find_container(containers: Vec<ContainerSummary>, args: Cli) {
    for container in containers {
        let container_name = container.names.map(|s| s[0].to_owned()).unwrap_or_default();
        let striped_container_name = container_name.trim_start_matches('/');

        if striped_container_name == args.name {
            println!(
                "{}\t{}\t{:?}\t{}\t{}",
                &container.id.unwrap_or_default(),
                container.image.unwrap_or_default(),
                container.state,
                container.status.unwrap_or_default(),
                striped_container_name
            );
            break;
        }
    }
}
