mod from_config;

/**
	This enum represents a single instance of configuration that needs user consent.

	It is implemented by individual configuration and the whole Config struct.

	The fields are not exposed, retrieve info from id and display_name. This also implements
	std::fmt::Display to present.
*/
pub enum DynamicPermission {
	Notifications,
	Inhibit,
	DGPU,
	Kvm,
	Input,
	Camera,
}

impl DynamicPermission {
	/**
		Retrieve the internal ID from a DynamicPermission
	*/
	pub fn id(&self) -> &str {
		match self {
			Self::Notifications	=> {
				"ipc.notifications"
			}
			Self::Inhibit		=> {
				"portals.inhibit"
			}
			Self::DGPU		=> {
				"device.dgpu"
			}
			Self::Kvm		=> {
				"device.kvm"
			}
			Self::Input		=> {
				"device.input"
			}
			Self::Camera		=> {
				"device.camera"
			}
		}
	}
}

impl std::fmt::Display for DynamicPermission {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DynamicPermission::Notifications	=> {
				f.write_str("Send notifications")
			}
			DynamicPermission::Inhibit	=> {
				f.write_str("Inhibit system suspend and shutdown")
			}
			DynamicPermission::DGPU		=> {
				f.write_str("Render on Discrete GPU")
			}
			DynamicPermission::Kvm		=> {
				f.write_str("Run a Kernel Virtual Machine")
			}
			DynamicPermission::Input	=> {
				f.write_str("Access and manage Input Devices")
			}
			DynamicPermission::Camera	=> {
				f.write_str("Access Camera")
			}
		}
	}
}
