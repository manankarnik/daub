hi clear
set background={mode}
if exists("syntax on")
    syntax reset
endif
let g:colors_name="daub"

" Core UI
hi! Normal guifg={base05} guibg={base00}
hi! NormalFloat guifg={base05} guibg={base01}
hi! CursorLine guibg={base01}
hi! Visual guibg={base03}
hi! LineNr guifg={base04}
hi! CursorLineNr guifg={base07}
hi! StatusLine guifg={base05} guibg={base01}
hi! StatusLineNC guifg={base05} guibg={base02}
hi! Search guifg={base00} guibg={base0A}
hi! Title guifg={base05}
hi! WinSeparator guifg={base02}
hi! EndOfBuffer guifg={base00}

" Basic syntax groups
hi! Comment guifg={base03} gui=italic
hi! Constant guifg={base09}
hi! String guifg={base0B}
hi! Character guifg={base08}
hi! Number guifg={base09}
hi! Boolean guifg={base09}
hi! Float guifg={base09}
hi! Operator guifg={base05}

hi! Identifier guifg={base07}
hi! Function guifg={base0D}

hi! Statement guifg={base0E}
hi! Conditional guifg={base0E}
hi! Repeat guifg={base0E}
hi! Label guifg={base0A}
hi! Keyword guifg={base0E}
hi! Exception guifg={base08}

hi! PreProc guifg={base0A}
hi! Include guifg={base0D}
hi! Define guifg={base08}
hi! Macro guifg={base09}
hi! PreCondit guifg={base0A}

hi! Type guifg={base0A}
hi! StorageClass guifg={base0A}
hi! Structure guifg={base0E}
hi! Typedef guifg={base0A}

hi! Special guifg={base0C}
hi! SpecialChar guifg={base0C}
hi! Tag guifg={base0A}
hi! Delimiter guifg={base04}
hi! SpecialComment guifg={base03}
hi! Debug guifg={base08}

hi! Underlined guifg={base0C} gui=underline
hi! Ignore guifg={base00}
hi! Error guifg={base08}
hi! Todo guifg={base0A} guibg={base01} gui=bold

hi! @keyword guifg={base0E} gui=italic
hi! @variable guifg={base05}
hi! @module guifg={base07} gui=italic
hi! @function.macro guifg={base0C}
hi! @lsp.type.parameter guifg={base0C}
hi! @lsp.type.interface guifg={base09}
hi! @lsp.type.macro guifg={base0E}

" Telescope
hi! TelescopeNormal guifg={base05}
hi! TelescopeBorder guifg={base02}
hi! TelescopePromptTitle guifg={base0B}
hi! TelescopeResultsTitle guifg={base0E}
hi! TelescopePreviewTitle guifg={base0A}
hi! TelescopePromptPrefix guifg={base08}
hi! TelescopeSelection guifg={base06} guibg={base03}
hi! TelescopeMatching guifg={base0A}
hi! TelescopeResultsIcon guifg={base05}
hi! TelescopePreviewLine guifg={base03} guibg={base01}
hi! TelescopePreviewChar guifg={base08} guibg={base01}
hi! TelescopePrompt guifg={base05} guibg={base01}

" Indent guides
hi! IndentBlanklineChar guifg={base03}
hi! IndentBlanklineSpaceChar guifg={base03}
hi! IndentBlanklineSpaceCharBlankline guifg={base03}

" Context highlight (for current scope)
hi! IblScope guifg={base0D}
hi! IndentBlanklineContextChar guifg={base05}
hi! IndentBlanklineContextChar guifg={base05}
hi! IndentBlanklineContextStart guisp={base05} gui=underline
hi! IndentBlanklineContextSpaceChar guifg={base05}

" Nvim Tree
hi! NvimTreeNormal guifg={base05} guibg={base01}
hi! NvimTreeFolderName guifg={base0D}
hi! link NvimTreeOpenedFolderName NvimTreeFolderName
hi! NvimTreeRootFolder guifg={base06}
hi! NvimTreeSymlink guifg={base0A}
hi! NvimTreeExecFile guifg={base0D}
hi! NvimTreeOpenedFile guifg={base0F}
hi! NvimTreeSpecialFile guifg={base09}
hi! NvimTreeIndentMarker guifg={base04}

" Which-key
hi! WhichKey guifg={base05}
hi! WhichKeyGroup guifg={base09}
hi! WhichKeyDesc guifg={base06}
hi! WhichKeySeparator guifg={base0A}
hi! WhichKeyFloating guifg={base0F}
hi! WhichKeyValue guifg={base0B}
hi! WhichKeyBorder guifg={base00}
