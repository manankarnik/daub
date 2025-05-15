vim.cmd("highlight clear")
if vim.fn.exists("syntax_on") then
	vim.cmd("syntax reset")
end
vim.g.background = "{mode}"
vim.g.colors_name = "daub"

return {{
	background = "{background}",
	background_alt = "{background_alt}",
	background_selection = "{background_selection}",
	foreground_invisible = "{foreground_invisible}",
	foreground_dark = "{foreground_dark}",
	foreground = "{foreground}",
	red = "{red}",
	orange = "{orange}",
	yellow = "{yellow}",
	green = "{green}",
	cyan = "{cyan}",
	blue = "{blue}",
	purple = "{purple}",
	brown = "{brown}",
}}
