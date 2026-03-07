vim.g.rustaceanvim = {
	server = {
		default_settings = {
			["rust-analyzer"] = {
				cargo = {
					extraEnv = {
						REST_USER = "",
						REST_USER_PASSWORD = "",
						WIFI_NETWORK = "",
						WIFI_PASSWORD = "",
						MEASUREMENTS_ENDPOINT = "",
					},
					allTargets = false,
					features = { "temperature" },
					allFeatures = false,
				},
			},
		},
	},
}
