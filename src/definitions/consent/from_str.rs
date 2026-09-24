impl TryFrom<&str> for super::DynamicPermission
{
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"ipc.notifications"	=> Ok(Self::Notifications),
			"portals.inhibit"	=> Ok(Self::Inhibit),
			"device.dgpu"		=> Ok(Self::DGPU),
			"device.kvm"		=> Ok(Self::Kvm),
			"device.input"		=> Ok(Self::Input),
			"device.camera"		=> Ok(Self::Camera),
			"lockdown.nolandlock"	=> Ok(Self::DisableLandlock),
			"ipc.mpris"		=> Ok(Self::MediaPlayer2(vec![])),
			"debugging"		=> Ok(Self::Debugging),
			v			=> {
				Err(
					ConversionError::UnknownID(v.into())
				)
			}
		}
	}

	type Error = ConversionError;
}

#[derive(thiserror::Error, Debug)]
pub enum ConversionError {
	#[error("Unknown id: {0}")]
	UnknownID(String),
}
