use std::collections::HashMap;
use std::fs::File;
use std::io::copy;
use std::time::Duration;
use log::{debug, error, info};
use reqwest::Client;

use crate::errors::GeneratorError;

#[derive(Default)]
pub struct SpigotServerGenerator<'a> {
	old_versions: HashMap<&'a str, &'a str>,
	cdn_versions: Vec<&'a str>
}

impl SpigotServerGenerator<'_> {
	fn new() -> Self {
		Self {
			old_versions: vec![
				("1.4.6", "spigot-1.4.6-R0.4-SNAPSHOT"),
			].into_iter().collect(),

			cdn_versions: vec![
				"1.4.6",
				"1.4.7",
				"1.5.1",
				"1.5.2",
				"1.6.2",
				"1.6.4",
				"1.7.2",
				"1.7.5",
				"1.7.8",
				"1.7.9",
				"1.7.10",
				"1.8",
				"1.8.3",
				"1.8.4",
				"1.8.5",
				"1.8.6",
				"1.8.7",
				"1.8.8",
				"1.9",
				"1.9.2",
				"1.9.4",
				"1.10",
				"1.10.2",
				"1.11",
				"1.11.1",
				"1.11.2",
				"1.12",
				"1.12.1",
				"1.12.2",
				"1.13",
				"1.13.1",
				"1.13.2",
				"1.14",
				"1.14.1",
				"1.14.2",
				"1.14.3",
				"1.14.4",
				"1.15",
				"1.15.1",
				"1.15.2",
				"1.16.1",
				"1.16.2",
				"1.16.3",
				"1.16.4",
				"1.16.5",
			]
		}
	}

	fn get_spiot_download_url(&self, version: &str) -> String {
		let converted_version = self.get_spigot_server_version(version);
		if self.cdn_versions.contains(&version) {
			format!("https://cdn.getbukkit.org/spigot/{}.jar", converted_version)
		} else {
			format!("https://download.getbukkit.org/spigot/{}.jar", converted_version)
		}
	}

	fn get_spigot_server_version(&self, version: &str) -> String {
		match self.old_versions.get(version) {
			Some(old_versions) => old_versions.to_string(),
			None => format!("spigot-{}", version),
		}
	}

	pub fn generate_server(&self, version: &str, folder: &str) -> Result<(), GeneratorError> {
		debug!("Generating server for version: {}, folder: {}", version, folder);
		let url = self.get_spiot_download_url(version);
		debug!("Download URL: {}", url);
		let path = format!("{}/spigot-{}.jar", folder, version);
		debug!("File path: {}", path);
		let client = Client::builder()
			.timeout(Duration::from_secs(60))
			.build()
			.map_err(GeneratorError::DownloadError)?;
		
		let response = match client.get(&url).send() {
			Ok(response) => response,
			Err(e) => {
				error!("Failed to get response: {:?}", e);
				return Err(GeneratorError::DownloadError(e));
			}
		};
		debug!("Response received");
		let mut file = File::create(path).map_err(GeneratorError::FileCreationError)?;
		debug!("File created");
		let content = response.bytes().map_err(GeneratorError::DownloadError)?;
		debug!("Content read");
		copy(&mut content.as_ref(), &mut file).map_err(GeneratorError::FileCreationError)?;
		debug!("Content copied to file");
		info!("Spigot server downloaded!");
		Ok(())
	}
}

#[cfg(test)]
mod test { 
	use super::*;
	use std::path::Path;
	use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            env_logger::init();
        });
    }

	#[test]
	fn get_spigot_server_version_should_returns_old_supported_version() {
		// GIVEN a spigot server
		let spigot_server = SpigotServerGenerator::new();
		// AND a version
		let old_version = "1.4.6";

		// WHEN I get the version
		let result = spigot_server.get_spigot_server_version(old_version);

		// THEN it should be converted
		assert_eq!("spigot-1.4.6-R0.4-SNAPSHOT", result);
	}

	#[test]
	fn get_spigot_server_version_should_returns_new_version() {
		// GIVEN a spigot server
		let spigot_server = SpigotServerGenerator::new();
		// AND a version
		let new_version = "1.11";

		// WHEN I get the version
		let result = spigot_server.get_spigot_server_version(new_version);

		// THEN it should be converted
		assert_eq!("spigot-1.11", result);
	}

	#[test]
	fn get_spiot_download_url_should_returns_old_url() {
		// GIVEN a spigot server
		let spigot_server = SpigotServerGenerator::new();
		// AND a version
		let old_version = "1.4.6";

		// WHEN I get the download url
		let result = spigot_server.get_spiot_download_url(old_version);

		// THEN it should be converted
		assert_eq!("https://cdn.getbukkit.org/spigot/spigot-1.4.6-R0.4-SNAPSHOT.jar", result);
	}

	#[test]
	fn get_spiot_download_url_should_returns_new_url() {
		// GIVEN a spigot server
		let spigot_server = SpigotServerGenerator::new();
		// AND a version
		let new_version = "1.18";

		// WHEN I get the download url
		let result = spigot_server.get_spiot_download_url(new_version);

		// THEN it should be converted
		assert_eq!("https://download.getbukkit.org/spigot/spigot-1.18.jar", result);
	}

	#[test]
	fn generate_server_should_generate_server() {
		init_logger();
		// GIVEN a spigot server
		let spigot_server = SpigotServerGenerator::new();
		// AND a version
		let version = "1.4.6";
		// AND a folder
		let folder = "/tmp";

		// WHEN im downloading the server
		let result = spigot_server.generate_server(version, folder);

		// THEN the server has to be downloaded
		let path = format!("{}/spigot-1.4.6.jar", folder);
		assert!(result.is_ok());
		assert!(Path::new(&path).exists());
		assert!(Path::new(&path).is_file());
	}
}
