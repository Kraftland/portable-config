pub mod from_config;
pub mod from_str;

/**
	This enum represents a single instance of configuration that needs user consent.

	It is implemented by individual configuration and the whole Config struct.

	The fields are not exposed, retrieve info from id and display_name. This also implements
	std::fmt::Display to present.
*/
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum DynamicPermission {
	Notifications,
	Inhibit,
	DGPU,
	Kvm,
	Input,
	Camera,
	DisableLandlock,
	MediaPlayer2,
	Debugging,
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
			Self::DisableLandlock	=> {
				"lockdown.nolandlock"
			}
			Self::MediaPlayer2	=> {
				"ipc.mpris"
			}
			Self::Debugging		=> {
				"debugging"
			}
		}
	}

	/**
		Allow list certain permissions because not granting will break apps.

		Currently only landlock.
	*/
	pub fn default_allow(&self) -> bool {
		match &self {
			Self::DisableLandlock	=> {
				true
			}
			_			=> {
				false
			}
		}
	}
}

impl std::fmt::Display for DynamicPermission {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use gettextrs::{textdomain, bind_textdomain_codeset};

		match textdomain("portable-config") {
			Ok(_)	=> {}
			Err(e)	=> {
				return f.write_fmt(
					format_args!(
						"Could not translate permission: {}, {}",
						self,
						e,
					)
				);
			}
		};

		match bind_textdomain_codeset("portable-config", "UTF-8") {
			Ok(_)	=> {}
			Err(e)	=> {
				return f.write_fmt(
					format_args!(
						"Could not translate permission: {}, {}",
						self,
						e,
					)
				);
			}
		};

		match self {
			DynamicPermission::Notifications	=> {
				f.write_str(
					gettextrs::gettext("Send notifications").as_str()
				)
			}
			DynamicPermission::Inhibit	=> {
				f.write_str(
					gettextrs::gettext("Inhibit system suspend and shutdown")
						.as_str()
				)
			}
			DynamicPermission::DGPU		=> {
				f.write_str(
					gettextrs::gettext("Render on Discrete GPU")
						.as_str()
				)
			}
			DynamicPermission::Kvm		=> {
				f.write_str(
					gettextrs::gettext(
						"Run a Kernel Virtual Machine"
					)
						.as_str()
				)
			}
			DynamicPermission::Input	=> {
				f.write_str(
					gettextrs::gettext("Access and manage Input Devices")
						.as_str()
				)
			}
			DynamicPermission::Camera	=> {
				f.write_str(
					gettextrs::gettext("Access Camera")
						.as_str()
				)
			}
			DynamicPermission::DisableLandlock
							=> {
				f.write_str(
					gettextrs::gettext(
						"Mount namespace"
					)
						.as_str()
				)
			}
			DynamicPermission::MediaPlayer2
							=> {
				f.write_str(
					gettextrs::gettext(
						"Control Media Players"
					)
						.as_str()
				)
			}
			DynamicPermission::Debugging	=> {
				f.write_str(
					gettextrs::gettext(
						"Perform debugging"
					)
						.as_str()
				)
			}
		}
	}
}
