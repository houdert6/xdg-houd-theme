/*

    An xdg desktop portal implementation for the Settings interface that sets permanent dark mode
    Copyright (C) 2026  houdert6

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

*/

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
