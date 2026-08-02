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
						MEASUREMENTS_SERVER_URL = "",
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

-- suppress RA rustc errors arising from use of the std crate: source = "rustc"
-- :lua vim.print(vim.diagnostic.get(0))
local orig = vim.lsp.handlers["textDocument/publishDiagnostics"]

vim.lsp.handlers["textDocument/publishDiagnostics"] = function(err, result, ctx, config)
	if not result or not result.uri then
		return orig(err, result, ctx, config)
	end

	local path = vim.uri_to_fname(result.uri)
	if not path or path == "" then
		return orig(err, result, ctx, config)
	end
	path = path:gsub("\\", "/")

	local rel = vim.fn.fnamemodify(path, ":.")
	rel = rel:gsub("\\", "/")
	if rel:match("^tests/test_.*%.rs$") then
		local filtered = {}
		for _, d in ipairs(result.diagnostics or {}) do
			if not (d.source and d.source:match("rust")) then
				table.insert(filtered, d)
			end
		end
		result.diagnostics = filtered
	end
	return orig(err, result, ctx, config)
end
