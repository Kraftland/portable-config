
impl crate::definitions::Config {
	/**
		Deserialise a Portable configuration from a Tokio file.
	*/
	pub async fn from_toml_file(mut file: tokio::fs::File)
	-> Result<crate::definitions::Config, crate::errors::ConfigError> {
		let content = {
			use tokio::io::AsyncReadExt;
			let mut buffer = String::new();
			file
				.read_to_string(&mut buffer)
				.await
				.map_err(crate::errors::ConfigError::ReadIOError)
				?;
			buffer
		};

		let config: crate::Config = toml::from_str(&content)
			.map_err(crate::errors::ConfigError::MalformedTOMLConfig)
			?;

		Ok(config)
	}
}
