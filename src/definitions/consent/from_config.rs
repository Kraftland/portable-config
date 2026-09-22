impl From<&crate::definitions::SysMgmt> for Vec<super::DynamicPermission> {
	fn from(value: &crate::definitions::SysMgmt) -> Self {
		let mut ret = vec![];

		if value.allow_inhibit {
			ret.push(super::DynamicPermission::Inhibit);
		};

		for perm in &value.device_allow {
			let dynamic_permission = match perm {
				crate::definitions::DeviceAllow::Kvm	=> {
					super::DynamicPermission::Kvm
				}
				crate::definitions::DeviceAllow::Input	=> {
					super::DynamicPermission::Input
				}
				crate::definitions::DeviceAllow::Camera	=> {
					super::DynamicPermission::Camera
				}
				crate::definitions::DeviceAllow::DiscreteGPU
									=> {
					super::DynamicPermission::DGPU
				}
			};
			ret.push(dynamic_permission);
		};
		ret
	}
}

impl From<&crate::definitions::Privacy> for Vec<super::DynamicPermission> {
	fn from(value: &crate::definitions::Privacy) -> Self {
		let mut ret = vec![];

		if ! value.lockdown_options.get_fine_grained().landlock {
			ret.push(super::DynamicPermission::DisableLandlock);
		};

		if value.classic_notif || value.push_notification {
			ret.push(super::DynamicPermission::Notifications);
		};

		ret
	}
}
