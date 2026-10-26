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

use serde::{Deserialize, Serialize};
use zbus::{interface, zvariant::{self, DeserializeDict, OwnedValue, SerializeDict, Type, Value}};

pub struct Settings  {

}

#[derive(Debug, Default, SerializeDict, DeserializeDict, Type)]
#[zvariant(signature = "a{sa{sv}}")]
pub struct SettingsReadAll {
    #[zvariant(rename = "org.freedesktop.appearance")]
    appearance: SettingsAppearance,
}

#[derive(Debug, Default, SerializeDict, DeserializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct SettingsAppearance {
    #[zvariant(rename = "color-scheme")]
    color_scheme: u32,
}

#[interface(
    name = "org.freedesktop.impl.portal.Settings"
)]
impl Settings {
    /// Read method
    fn read(&self, namespace: &str, key: &str) -> OwnedValue {
        if namespace == "org.freedesktop.appearance" && key == "color-scheme" {
            OwnedValue::try_from(Value::U32(1)).unwrap()
        } else {
            OwnedValue::try_from(Value::U32(0)).unwrap()
        }
    }

    /// ReadAll method
    fn read_all(
        &self,
        namespaces: Vec<String>,
    ) -> SettingsReadAll {
        SettingsReadAll {appearance: SettingsAppearance {color_scheme: 1}}
    }

    /// version property
    #[zbus(property, name = "version")]
    fn version(&self) -> u32 {
        2
    }
}
