hi! clear
set background={mode}
if exists("syntax on")
    syntax reset
endif
let g:colors_name="daub"

" =============================================
" Core UI Elements
" =============================================
hi! Normal guifg={base05} guibg={base00}
hi! NormalFloat guifg={base05} guibg={base01}
hi! CursorLine guibg={base01}
hi! Visual guibg={base02}
hi! LineNr guifg={base03}
hi! CursorLineNr guifg={base07}
hi! StatusLine guifg={base04} guibg={base01}
hi! StatusLineNC guifg={base03} guibg={base01}
hi! Search guifg={base05} guibg={base02}
hi! CurSearch guifg={base01} guibg={base0A}
hi! IncSearch guifg={base01} guibg={base0C}
hi! Title guifg={base0D}
hi! WinSeparator guifg={base02}
hi! EndOfBuffer guifg={base00}

" =============================================
" Basic Syntax Highlighting
" =============================================
" Comments and documentation
hi! Comment guifg={base03} gui=italic

" Constants
hi! Constant guifg={base09}
hi! String guifg={base0B}
hi! Character guifg={base08}
hi! Number guifg={base09}
hi! Boolean guifg={base09}
hi! Float guifg={base09}

" Identifiers
hi! Identifier guifg={base07}
hi! Function guifg={base0D}
hi! Operator guifg={base05}

" Control flow and statements
hi! Statement guifg={base08}
hi! Conditional guifg={base0E}
hi! Repeat guifg={base0A}
hi! Label guifg={base0A}
hi! Keyword guifg={base0E}
hi! Exception guifg={base08}

" Preprocessor directives
hi! PreProc guifg={base0A}
hi! Include guifg={base0D}
hi! Define guifg={base0E}
hi! Macro guifg={base0C}
hi! PreCondit guifg={base0A}

" Type definitions
hi! Type guifg={base0A}
hi! StorageClass guifg={base0A}
hi! Structure guifg={base0E}
hi! Typedef guifg={base0A}

" Special elements
hi! Special guifg={base0F}
hi! SpecialChar guifg={base0F}
hi! Tag guifg={base0A}
hi! Delimiter guifg={base04}
hi! SpecialComment guifg={base03}
hi! Debug guifg={base08}

" Spell
hi! SpellBad guisp={base08} gui=undercurl
hi! SpellCap guisp={base0A} gui=undercurl
hi! SpellRare guisp={base0C} gui=undercurl
hi! SpellLocal guisp={base0B} gui=undercurl

" =============================================
" Text Formatting and Error States
" =============================================
hi! Underlined guifg={base08} gui=underline
hi! Ignore guifg={base00}
hi! Error guifg={base00} guibg={base08}
hi! Todo guifg={base0A} guibg={base01} gui=bold
hi! Bold gui=bold
hi! Italic gui=italic

" =============================================
" UI Components and Feedback
" =============================================
hi! Directory guifg={base0D}
hi! ErrorMsg guifg={base08} guibg={base00}
hi! FoldColumn guifg={base0C} guibg={base01}
hi! Folded guifg={base03} guibg={base01}
hi! MatchParen guifg={base09} guibg={base02} gui=bold
hi! ModeMsg guifg={base05} gui=bold
hi! MoreMsg guifg={base05} gui=bold
hi! Question guifg={base0D}
hi! Substitute guifg={base01} guibg={base0A}
hi! SpecialKey guifg={base03}
hi! TooLong guifg={base08}
hi! VisualNOS guifg={base08}
hi! WarningMsg guifg={base08}
hi! WildMenu guifg={base08} guibg={base0A}
hi! Conceal guifg={base0D} guibg={base00}
hi! Cursor guifg={base00} guibg={base07}
hi! NonText guifg={base03}
hi! SignColumn guifg={base03}
hi! ColorColumn guibg={base01}
hi! CursorColumn guibg={base01}
hi! QuickFixLine guifg={base0A} guibg={base01}
"
" =============================================
" Diff
" =============================================
hi! Added guifg={base0B}
hi! Changed guifg={base0A}
hi! Removed guifg={base08}
hi! DiffAdd guifg={base0B} guibg=NONE
hi! DiffChange guifg={base0A} guibg=NONE
hi! DiffDelete guifg={base08} guibg=NONE
hi! DiffText guifg={base0C} guibg=NONE

" =============================================
" Menus and Navigation
" =============================================
hi! PMenu guifg={base05} guibg={base01}
hi! PMenuSel guifg={base05} guibg={base01}
hi! PMenuThumb guibg={base02}
hi! TabLine guifg={base03} guibg={base01}
hi! TabLineFill guifg={base03} guibg={base01}
hi! TabLineSel guifg={base04} guibg={base01} gui=NONE

" =============================================
" LSP and Tree-sitter Highlighting
" =============================================
hi! @keyword guifg={base0E} gui=italic
hi! @variable guifg={base05}
hi! @module guifg={base07} gui=italic
hi! @lsp.type.parameter guifg={base08}
hi! link @lsp.type.interface Type
hi! link @lsp.type.decorator Constant
hi! link @lsp.type.macro Macro
hi! link @lsp.type.formatSpecifier Special
hi! link @function.macro Macro

" =============================================
" Plugin: Telescope
" =============================================
hi! TelescopeNormal guifg={base05}
hi! TelescopeBorder guifg={base02}
hi! TelescopePromptTitle guifg={base0B}
hi! TelescopeResultsTitle guifg={base0E}
hi! TelescopePreviewTitle guifg={base0A}
hi! TelescopePromptPrefix guifg={base08}
hi! TelescopeSelection guifg={base06} guibg={base02}
hi! TelescopeMatching guifg={base0A}
hi! TelescopeResultsIcon guifg={base05}
hi! TelescopePreviewLine guifg={base03} guibg={base01}
hi! TelescopePreviewChar guifg={base08} guibg={base01}
hi! TelescopePrompt guifg={base05} guibg={base01}

" =============================================
" Plugin: Indent Guides
" =============================================
hi! IblIndent guifg={base03}
hi! IblScope guifg={base0D}
hi! IblWhitespace guifg={base00}

" =============================================
" Plugin: Nvim Tree
" =============================================
hi! NvimTreeNormal guifg={base05} guibg={base00}
hi! NvimTreeFolderName guifg={base0D}
hi! link NvimTreeOpenedFolderName NvimTreeFolderName
hi! NvimTreeRootFolder guifg={base06}
hi! NvimTreeSymlink guifg={base0A}
hi! NvimTreeExecFile guifg={base0D}
hi! NvimTreeOpenedFile guifg={base0F}
hi! NvimTreeSpecialFile guifg={base09}
hi! NvimTreeIndentMarker guifg={base04}

" =============================================
" Plugin: Which-Key
" =============================================
hi! WhichKeyGroup guifg={base0D}
hi! WhichKeyDesc guifg={base05}
hi! WhichKeySeparator guifg={base0A}

" =============================================
" Plugin: Gitsigns
" =============================================
hi! GitSignsAdd guifg={base0B}
hi! GitSignsChange guifg={base0A}
hi! GitSignsDelete guifg={base08}
hi! GitSignsCurrentLineBlame guifg={base03}
hi! link GitSignsStagedAdd GitSignsAdd
hi! link GitSignsStagedChange GitSignsChange
hi! link GitSignsStagedDelete GitSignsDelete
hi! link GitSignsStagedChangedelete GitSignsChange
hi! link GitSignsStagedTopdelete GitSignsDelete
hi! link GitSignsStagedUntracked GitSignsAdd
hi! link GitSignsStagedAddNr GitSignsAdd
hi! link GitSignsStagedChangeNr GitSignsChange
hi! link GitSignsStagedDeleteNr GitSignsDelete
hi! link GitSignsStagedChangedeleteNr GitSignsChange
hi! link GitSignsStagedTopdeleteNr GitSignsDelete
hi! link GitSignsStagedUntrackedNr GitSignsAdd
hi! link GitSignsStagedAddLn GitSignsAdd
hi! link GitSignsStagedChangeLn GitSignsChange
hi! link GitSignsStagedChangedeleteLn GitSignsChange
hi! link GitSignsStagedUntrackedLn GitSignsAdd
hi! link GitSignsStagedAddCul GitSignsAdd
hi! link GitSignsStagedChangeCul GitSignsChange
hi! link GitSignsStagedDeleteCul GitSignsDelete
hi! link GitSignsStagedChangedeleteCul GitSignsChange
hi! link GitSignsStagedTopdeleteCul GitSignsDelete
hi! link GitSignsStagedUntrackedCul GitSignsAdd

" =============================================
" Plugin: LSP Diagnostics
" =============================================
hi! DiagnosticError guifg={base08}
hi! DiagnosticWarn guifg={base0A}
hi! DiagnosticInfo guifg={base0D}
hi! DiagnosticHint guifg={base0C}
hi! DiagnosticOk guifg={base0B}
hi! DiagnosticUnderlineError guifg={base08} gui=underline
hi! DiagnosticUnderlineWarn guifg={base0A} gui=underline
hi! DiagnosticUnderlineInfo guifg={base0D} gui=underline
hi! DiagnosticUnderlineHint guifg={base0C} gui=underline
hi! LspReferenceText guibg={base02}
hi! LspReferenceRead guibg={base02}
hi! LspReferenceWrite guibg={base02}
hi! LspCodeLens guifg={base03} gui=italic

" =============================================
" Plugin: Treesitter
" =============================================
hi! @constructor guifg={base0D}
hi! @tag.delimiter guifg={base0F}
hi! @tag.attribute guifg={base0A}
hi! @text.title guifg={base0D} gui=bold
hi! @text.uri guifg={base0D} gui=underline
hi! @text.literal guifg={base0B}
hi! @text.reference guifg={base0C}
hi! @text.todo guifg={base0D} guibg={base01}
hi! @text.note guifg={base0B} guibg={base01}
hi! @text.warning guifg={base0A} guibg={base01}
hi! @text.danger guifg={base08} guibg={base01}
hi! link @variable.builtin Variable
hi! link @function.builtin Function

" =============================================
" Plugin: Cmp (Completion)
" =============================================
hi! CmpItemAbbrDeprecated guifg={base03} gui=strikethrough
hi! CmpItemAbbrMatch guifg={base0D}
hi! CmpItemAbbrMatchFuzzy guifg={base0D}
hi! CmpItemKind guifg={base05}
hi! CmpItemKindVariable guifg={base0E}
hi! CmpItemKindInterface guifg={base0A}
hi! CmpItemKindText guifg={base0B}
hi! CmpItemKindFunction guifg={base0D}
hi! CmpItemKindMethod guifg={base0D}
hi! CmpItemKindKeyword guifg={base0E}
hi! CmpItemKindProperty guifg={base0A}
hi! CmpItemKindUnit guifg={base0C}

" =============================================
" Plugin: Notify
" =============================================
hi! NotifyERRORBorder guifg={base08}
hi! NotifyWARNBorder guifg={base0A}
hi! NotifyINFOBorder guifg={base0D}
hi! NotifyDEBUGBorder guifg={base03}
hi! NotifyTRACEBorder guifg={base0C}
hi! NotifyERRORIcon guifg={base08}
hi! NotifyWARNIcon guifg={base0A}
hi! NotifyINFOIcon guifg={base0D}
hi! NotifyDEBUGIcon guifg={base03}
hi! NotifyTRACEIcon guifg={base0C}
hi! NotifyERRORTitle guifg={base08}
hi! NotifyWARNTitle guifg={base0A}
hi! NotifyINFOTitle guifg={base0D}
hi! NotifyDEBUGTitle guifg={base03}
hi! NotifyTRACETitle guifg={base0C}

" =============================================
" Plugin: Bufferline
" =============================================
hi! BufferLineFill guibg={base01}
hi! BufferLineBackground guifg={base03} guibg={base01}
hi! BufferLineBufferVisible guifg={base05} guibg={base01}
hi! BufferLineBufferSelected guifg={base06} guibg={base02} gui=bold
hi! BufferLineTab guifg={base03} guibg={base01}
hi! BufferLineTabSelected guifg={base06} guibg={base02}
hi! BufferLineTabClose guifg={base08} guibg={base01}
hi! BufferLineIndicatorSelected guifg={base0B} guibg={base02}
hi! BufferLineSeparator guifg={base02} guibg={base01}
hi! BufferLineSeparatorSelected guifg={base02} guibg={base02}
hi! link BufferLineTabSeparator BufferLineSeparator
hi! link BufferLineTabSeparatorSelected BufferLineSeparatorSelected
hi! BufferLineModified guifg={base0A} guibg={base01}
hi! BufferLineModifiedVisible guifg={base0A} guibg={base01}
hi! BufferLineModifiedSelected guifg={base0A} guibg={base02}
hi! BufferLineCloseButton guifg={base06} guibg={base01}
hi! BufferLineCloseButtonVisible guifg={base06} guibg={base01}
hi! BufferLineCloseButtonSelected guifg={base06} guibg={base02}

" =============================================
" Plugin: Dashboard
" =============================================
hi! DashboardHeader guifg={base0D}
hi! DashboardCenter guifg={base0B}
hi! DashboardShortcut guifg={base0E}
hi! DashboardFooter guifg={base03}

" =============================================
" Plugin: Barbar
" =============================================
hi! BufferCurrent guifg={base06} guibg={base02}
hi! BufferCurrentIndex guifg={base0B} guibg={base02}
hi! BufferCurrentMod guifg={base0A} guibg={base02}
hi! BufferCurrentSign guifg={base0B} guibg={base02}
hi! BufferCurrentTarget guifg={base08} guibg={base02} gui=bold
hi! BufferVisible guifg={base05} guibg={base01}
hi! BufferVisibleIndex guifg={base05} guibg={base01}
hi! BufferVisibleMod guifg={base0A} guibg={base01}
hi! BufferVisibleSign guifg={base05} guibg={base01}
hi! BufferVisibleTarget guifg={base08} guibg={base01} gui=bold
hi! BufferInactive guifg={base03} guibg={base01}
hi! BufferInactiveIndex guifg={base03} guibg={base01}
hi! BufferInactiveMod guifg={base0A} guibg={base01}
hi! BufferInactiveSign guifg={base03} guibg={base01}
hi! BufferInactiveTarget guifg={base08} guibg={base01} gui=bold

" =============================================
" Plugin: Todo comments
" =============================================
" PERF
hi! TodoBgPERF guifg={base00} guibg={base0E}
hi! TodoFgPERF guifg={base0E}
hi! TodoSignPERF guifg={base0E}

" HACK
hi! TodoBgHACK guifg={base00} guibg={base09}
hi! TodoFgHACK guifg={base09}
hi! TodoSignHACK guifg={base09}

" TODO
hi! TodoBgTODO guifg={base00} guibg={base0D}
hi! TodoFgTODO guifg={base0D}
hi! TodoSignTODO guifg={base0D}

" NOTE
hi! TodoBgNOTE guifg={base00} guibg={base0B}
hi! TodoFgNOTE guifg={base0B} gui=bold
hi! TodoSignNOTE guifg={base0B}

" FIX
hi! TodoBgFIX guifg={base00} guibg={base08}
hi! TodoFgFIX guifg={base08} gui=bold
hi! TodoSignFIX guifg={base08}

" WARN
hi! TodoBgWARN guifg={base00} guibg={base0A}
hi! TodoFgWARN guifg={base0A}
hi! TodoSignWARN guifg={base0A}

" TEST
hi! TodoBgTEST guifg={base00} guibg={base0E}
hi! TodoFgTEST guifg={base0E}
hi! TodoSignTEST guifg={base0E}
