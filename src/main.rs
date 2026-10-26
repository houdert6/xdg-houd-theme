use std::future;

use tracing::info;
use zbus::conn::Builder;

use crate::settings::Settings;

mod settings;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings {};
    let conn = Builder::session()?
        .name("org.freedesktop.impl.portal.desktop.houdtheme")?
        .serve_at("/org/freedesktop/portal/desktop", settings)?
        .build()
        .await?;
    info!("Started portal on {}",
        conn.unique_name().map_or("no unique_name", |name| name.as_str()));
    // Just wait forever lol
    future::pending::<()>().await;
    Ok(())
}
