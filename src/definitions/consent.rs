
/**
	The Consent struct represents a single instance of configuration that needs user consent.

	It is implemented by individual configuration and the whole Config struct.
*/
pub struct DynamicPermission {
	pub id:			String,
	pub display_name:	String,
}

/**
	A permission type displayed to user.

	It implements the std::fmt::Display trait for display.
*/
pub enum PermissionType {
	DBusSession,
}

impl std::fmt::Display for PermissionType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::DBusSession	=> {
				f.write_str("Session Bus")
			}
		}
	}
}
