vim.cmd("highlight clear")
if vim.fn.exists("syntax_on") then
	vim.cmd("syntax reset")
end
vim.g.background = "{mode}"
vim.g.colors_name = "daub"

return {{
		lv00 = "{lv00}",
		lv01 = "{lv01}",
		lv02 = "{lv02}",
		lv03 = "{lv03}",
		lv04 = "{lv04}",
		lv05 = "{lv05}",
		clrd = "{clrd}",
		clor = "{clor}",
		clyl = "{clyl}",
		clgn = "{clgn}",
		clcy = "{clcy}",
		clbl = "{clbl}",
		clmg = "{clmg}",
		clbn = "{clbn}",
}}
