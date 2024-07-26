use askeladd::config::Settings;
use askeladd::dvm::service_provider::ServiceProvider;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ******************************************************
    // ****************** SETUP *****************************
    // ******************************************************
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    dotenv().ok();
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
