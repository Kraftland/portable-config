#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
	#[error("I/O error while reading file: {0:#?}")]
	ReadIOError(
		std::io::Error
	),
	#[error("Malformed TOML configuration file: {0:#?}")]
	MalformedTOMLConfig(toml::de::Error),

	#[error("Malformed Bash-style legacy configuration file: {0:#?}")]
	MalformedBashConfig(portable_legacy_conf::Error),
}
