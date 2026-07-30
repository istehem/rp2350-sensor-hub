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

local function run_test_current()
	local filename = vim.fn.expand("%:t")
	local name = filename:match("^test_(.-)%.rs$")
	if not name then
		vim.notify("Not a tests/test_*.rs file: " .. filename, vim.log.levels.WARN)
		return
	end

	local cmd = table.concat({
		"just test",
		name,
	}, " ")

	vim.cmd("terminal " .. cmd)
end

vim.keymap.set("n", "<leader>tt", run_test_current, { desc = "Run current tests/test_*.rs" })
