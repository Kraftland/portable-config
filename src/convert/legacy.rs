impl From<portable_legacy_conf::Config> for crate::Config {
	fn from(value: portable_legacy_conf::Config) -> Self {
		let device_allow = {
			use crate::definitions::DeviceAllow;

			let mut vec = vec![];

			if value.game {
				vec.push(DeviceAllow::DiscreteGPU);
			}

			if value.camera {
				vec.push(DeviceAllow::Camera);
			}

			if value.input_dev {
				vec.push(DeviceAllow::Input);
			}
			vec
		};

		crate::Config {
			metadata:		crate::definitions::Metadata {
				sandbox_id:		value.app_id,
				display_name:		value.friendly_name,
				state_directory:	value.state_dir,
			},
			exec:			crate::definitions::Exec {
				target:			value.target.0,
				arguments:		value.target.1.unwrap_or(vec![]),
				overlay:		false,
			},
			dbus_activation:	crate::definitions::BusExec {
				enable:			false,
				target:			String::new(),
				arguments:		vec![],
			},
			system:			crate::definitions::SysMgmt {
				allow_inhibit:		false,
				conduct_inhibit:	false,
				uclamp_max:		100,
				device_allow:		device_allow,
			},
			network:		crate::definitions::Network {
				allow_network:		value.bind_network,
				enable_filter:		false,
				block_dest:		vec![],
			},
			privacy:		crate::definitions::Privacy {
				lockdown_options:	crate::definitions::LockdownConfig::default(),
				x11_compat:		! value.wayland,
				classic_notif:		false,
				push_notification:	false,
			},
			advanced:		crate::definitions::Advanced {
				use_zink:		value.zink,
				qt5_compat:		value.qt5,
				mpris_names:		vec![],
				tray_wake:		value.tray_wake,
				allow_kde_status:	false,
				flatpak_env:		value.flatpak_info,
				allow_debug:		false,
			},
		}
	}
}
