vim.g.rustaceanvim = {
	server = {
		default_settings = {
			["rust-analyzer"] = {
				cargo = {
					extraEnv = {
						REST_USER = "",
						REST_USER_PASSWORD = "",
					},
					allTargets = false,
				},
			},
		},
	},
}
