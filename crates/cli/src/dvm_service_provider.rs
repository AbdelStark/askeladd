use std::io::Write;

use askeladd::config::Settings;
use askeladd::dvm::service_provider::ServiceProvider;
use colored::*;
use dotenv::dotenv;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ******************************************************
    // ****************** SETUP *****************************
    // ******************************************************
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
                    .blue(),
                record.level().to_string().yellow(),
                record.args()
            )
        })
        .init();

    let settings = Settings::new().expect("Failed to load settings");

    // ******************************************************
    // ****************** INIT SERVICE PROVIDER *************
    // ******************************************************
    let mut service_provider = ServiceProvider::new(settings)?;
    service_provider.init().await?;

    // ******************************************************
    // ****************** RUN SERVICE PROVIDER **************
    // ******************************************************
    service_provider.run().await?;

    Ok(())
}
