hi! clear
set background={mode}
if exists("syntax on")
    syntax reset
endif
let g:colors_name="daub"

" =============================================
" Core UI Elements
" =============================================
hi! Normal guifg={foreground} guibg={background}
hi! NormalFloat guifg={foreground} guibg={background_alt}
hi! CursorLine guibg={background_alt}
hi! Visual guibg={background_selection}
hi! LineNr guifg={foreground_invisible}
hi! CursorLineNr guifg={cursor_line}
hi! StatusLine guifg={foreground} guibg={background_alt}
hi! StatusLineNC guifg={foreground_invisible} guibg={background_alt}
hi! Search guifg={foreground} guibg={background_selection}
hi! CurSearch guifg={background_alt} guibg={yellow}
hi! IncSearch guifg={background_alt} guibg={cyan}
hi! Title guifg={blue}
hi! WinSeparator guifg={background_selection}
hi! EndOfBuffer guifg={background}

" =============================================
" Basic Syntax Highlighting
" =============================================
" Comments and documentation
hi! Comment guifg={comment} gui=italic

" Constants
hi! Constant guifg={constant}
hi! String guifg={string}
hi! Character guifg={red}
hi! link Number Constant
hi! link Boolean Constant
hi! link Float Constant

" Identifiers
hi! Identifier guifg={identifier}
hi! Function guifg={function}
hi! link Operator Normal

" Control flow and statements
hi! Keyword guifg={keyword}
hi! link Statement Keyword
hi! link Conditional Keyword
hi! link Repeat Keyword
hi! Label guifg={yellow}
hi! Exception guifg={red}

" Preprocessor directives
hi! link PreProc Type
hi! link Include Keyword
hi! link Define Keyword
hi! Macro guifg={macro}
hi! link PreCondit Type

" Type definitions
hi! Type guifg={type}
hi! link StorageClass Type
hi! link Structure Type
hi! link Typedef Type

" Special elements
hi! Special guifg={brown}
hi! link SpecialChar Special
hi! Tag guifg={yellow}
hi! link Delimiter Normal
hi! link SpecialComment Special
hi! Debug guifg={red}

" Spell
hi! SpellBad guisp={red} gui=undercurl
hi! SpellCap guisp={yellow} gui=undercurl
hi! SpellRare guisp={cyan} gui=undercurl
hi! SpellLocal guisp={green} gui=undercurl

" =============================================
" Text Formatting and Error States
" =============================================
hi! Underlined guifg={red} gui=underline
hi! Ignore guifg={background}
hi! Error guifg={background} guibg={red}
hi! Todo guifg={yellow} guibg={background_alt} gui=bold
hi! Bold gui=bold
hi! Italic gui=italic

" =============================================
" UI Components and Feedback
" =============================================
hi! Directory guifg={blue}
hi! ErrorMsg guifg={red} guibg={background}
hi! FoldColumn guifg={cyan} guibg={background_alt}
hi! Folded guifg={foreground_invisible} guibg={background_alt}
hi! MatchParen guifg={orange} guibg={background_selection} gui=bold
hi! ModeMsg guifg={foreground} gui=bold
hi! MoreMsg guifg={foreground} gui=bold
hi! Question guifg={blue}
hi! Substitute guifg={background_alt} guibg={yellow}
hi! SpecialKey guifg={foreground_invisible}
hi! TooLong guifg={red}
hi! VisualNOS guifg={red}
hi! WarningMsg guifg={red}
hi! WildMenu guifg={red} guibg={yellow}
hi! Conceal guifg={blue} guibg={background}
hi! Cursor guifg={background} guibg={cursor}
hi! NonText guifg={foreground_invisible}
hi! SignColumn guifg={foreground_invisible}
hi! ColorColumn guibg={background_alt}
hi! CursorColumn guibg={background_alt}
hi! QuickFixLine guifg={yellow} guibg={background_alt}
"
" =============================================
" Diff
" =============================================
hi! Added guifg={green}
hi! Changed guifg={yellow}
hi! Removed guifg={red}
hi! DiffAdd guifg={green} guibg=NONE
hi! DiffChange guifg={yellow} guibg=NONE
hi! DiffDelete guifg={red} guibg=NONE
hi! DiffText guifg={cyan} guibg=NONE

" =============================================
" Menus and Navigation
" =============================================
hi! PMenu guifg={foreground} guibg={background_alt}
hi! PMenuSel guifg={foreground} guibg={background_alt}
hi! PMenuThumb guibg={background_selection}
hi! TabLine guifg={foreground_invisible} guibg={background_alt}
hi! TabLineFill guifg={foreground_invisible} guibg={background_alt}
hi! TabLineSel guifg={foreground_dark} guibg={background_alt} gui=NONE

" =============================================
" LSP and Tree-sitter Highlighting
" =============================================
hi! @module guifg={cyan}
hi! @lsp.type.parameter guifg={foreground}
hi! @lsp.type.property guifg={foreground}
hi! link @variable Identifier
hi! link @keyword Keyword
hi! link @lsp.type.interface Type
hi! link @lsp.type.decorator Constant
hi! link @lsp.type.macro Macro
hi! link @lsp.type.formatSpecifier Special
hi! link @function.macro Macro

" =============================================
" Plugin: Telescope
" =============================================
hi! TelescopeNormal guifg={foreground}
hi! TelescopeBorder guifg={background_selection}
hi! TelescopePromptTitle guifg={green}
hi! TelescopeResultsTitle guifg={purple}
hi! TelescopePreviewTitle guifg={yellow}
hi! TelescopePromptPrefix guifg={red}
hi! TelescopeSelection guibg={background_alt}
hi! TelescopeMatching guifg={yellow}
hi! TelescopeResultsIcon guifg={foreground}
hi! TelescopePreviewLine guifg={foreground_invisible} guibg={background_alt}
hi! TelescopePreviewChar guifg={red} guibg={background_alt}
hi! TelescopePrompt guifg={foreground} guibg={background_alt}

" =============================================
" Plugin: Indent Guides
" =============================================
hi! IblIndent guifg={foreground_invisible}
hi! IblScope guifg={blue}
hi! IblWhitespace guifg={background}

" =============================================
" Plugin: Nvim Tree
" =============================================
hi! NvimTreeNormal guifg={foreground} guibg={background}
hi! NvimTreeFolderName guifg={blue}
hi! link NvimTreeOpenedFolderName NvimTreeFolderName
hi! NvimTreeRootFolder guifg={foreground}
hi! NvimTreeSymlink guifg={yellow}
hi! NvimTreeExecFile guifg={blue}
hi! NvimTreeOpenedFile guifg={brown}
hi! NvimTreeSpecialFile guifg={orange}
hi! NvimTreeIndentMarker guifg={foreground_dark}

" =============================================
" Plugin: Which-Key
" =============================================
hi! WhichKeyGroup guifg={blue}
hi! WhichKeyDesc guifg={foreground}
hi! WhichKeySeparator guifg={yellow}

" =============================================
" Plugin: Gitsigns
" =============================================
hi! GitSignsAdd guifg={green}
hi! GitSignsChange guifg={yellow}
hi! GitSignsDelete guifg={red}
hi! GitSignsCurrentLineBlame guifg={foreground_invisible}
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
hi! DiagnosticError guifg={red}
hi! DiagnosticWarn guifg={yellow}
hi! DiagnosticInfo guifg={blue}
hi! DiagnosticHint guifg={cyan}
hi! DiagnosticOk guifg={green}
hi! DiagnosticUnderlineError guifg={red} gui=underline
hi! DiagnosticUnderlineWarn guifg={yellow} gui=underline
hi! DiagnosticUnderlineInfo guifg={blue} gui=underline
hi! DiagnosticUnderlineHint guifg={cyan} gui=underline
hi! LspReferenceText guibg={background_selection}
hi! LspReferenceRead guibg={background_selection}
hi! LspReferenceWrite guibg={background_selection}
hi! LspCodeLens guifg={foreground_invisible} gui=italic

" =============================================
" Plugin: Treesitter
" =============================================
hi! @constructor guifg={blue}
hi! @tag.delimiter guifg={brown}
hi! @tag.attribute guifg={yellow}
hi! @text.title guifg={blue} gui=bold
hi! @text.uri guifg={blue} gui=underline
hi! @text.literal guifg={green}
hi! @text.reference guifg={cyan}
hi! @text.todo guifg={blue} guibg={background_alt}
hi! @text.note guifg={green} guibg={background_alt}
hi! @text.warning guifg={yellow} guibg={background_alt}
hi! @text.danger guifg={red} guibg={background_alt}
hi! @variable.builtin guifg={builtin}
hi! @function.builtin guifg={builtin}
hi! @type.builtin guifg={builtin}

" =============================================
" Plugin: Cmp (Completion)
" =============================================
hi! CmpItemAbbrDeprecated guifg={foreground_invisible} gui=strikethrough
hi! CmpItemAbbrMatch guifg={blue}
hi! CmpItemAbbrMatchFuzzy guifg={blue}
hi! CmpItemKind guifg={foreground}
hi! link CmpItemKindConstant Constant
hi! link CmpItemKindVariable Variable
hi! link CmpItemKindInterface Type
hi! link CmpItemKindEnum Type
hi! link CmpItemKindModule @module
hi! link CmpItemKindText Normal
hi! link CmpItemKindFunction Function
hi! link CmpItemKindMethod Function
hi! link CmpItemKindKeyword Keyword
hi! link CmpItemKindStruct Type
hi! link CmpItemKindSnippet String
hi! link CmpItemKindProperty @lsp.type.property
hi! CmpItemKindUnit guifg={cyan}

" =============================================
" Plugin: Notify
" =============================================
hi! NotifyERRORBorder guifg={red}
hi! NotifyWARNBorder guifg={yellow}
hi! NotifyINFOBorder guifg={blue}
hi! NotifyDEBUGBorder guifg={foreground_invisible}
hi! NotifyTRACEBorder guifg={cyan}
hi! NotifyERRORIcon guifg={red}
hi! NotifyWARNIcon guifg={yellow}
hi! NotifyINFOIcon guifg={blue}
hi! NotifyDEBUGIcon guifg={foreground_invisible}
hi! NotifyTRACEIcon guifg={cyan}
hi! NotifyERRORTitle guifg={red}
hi! NotifyWARNTitle guifg={yellow}
hi! NotifyINFOTitle guifg={blue}
hi! NotifyDEBUGTitle guifg={foreground_invisible}
hi! NotifyTRACETitle guifg={cyan}

" =============================================
" Plugin: Bufferline
" =============================================
hi! BufferLineFill guibg={background_alt}
hi! BufferLineBackground guifg={foreground_invisible} guibg={background_alt}
hi! BufferLineBufferVisible guifg={foreground} guibg={background_alt}
hi! BufferLineBufferSelected guifg={foreground} guibg={background_selection} gui=bold
hi! BufferLineTab guifg={foreground_invisible} guibg={background_alt}
hi! BufferLineTabSelected guifg={foreground} guibg={background_selection}
hi! BufferLineTabClose guifg={red} guibg={background_alt}
hi! BufferLineIndicatorSelected guifg={green} guibg={background_selection}
hi! BufferLineSeparator guifg={background_selection} guibg={background_alt}
hi! BufferLineSeparatorSelected guifg={background_selection} guibg={background_selection}
hi! link BufferLineTabSeparator BufferLineSeparator
hi! link BufferLineTabSeparatorSelected BufferLineSeparatorSelected
hi! BufferLineModified guifg={yellow} guibg={background_alt}
hi! BufferLineModifiedVisible guifg={yellow} guibg={background_alt}
hi! BufferLineModifiedSelected guifg={yellow} guibg={background_selection}
hi! BufferLineCloseButton guifg={foreground} guibg={background_alt}
hi! BufferLineCloseButtonVisible guifg={foreground} guibg={background_alt}
hi! BufferLineCloseButtonSelected guifg={foreground} guibg={background_selection}

" =============================================
" Plugin: Dashboard
" =============================================
hi! DashboardHeader guifg={blue}
hi! DashboardCenter guifg={green}
hi! DashboardShortcut guifg={purple}
hi! DashboardFooter guifg={foreground_invisible}

" =============================================
" Plugin: Barbar
" =============================================
hi! BufferCurrent guifg={foreground} guibg={background_selection}
hi! BufferCurrentIndex guifg={green} guibg={background_selection}
hi! BufferCurrentMod guifg={yellow} guibg={background_selection}
hi! BufferCurrentSign guifg={green} guibg={background_selection}
hi! BufferCurrentTarget guifg={red} guibg={background_selection} gui=bold
hi! BufferVisible guifg={foreground} guibg={background_alt}
hi! BufferVisibleIndex guifg={foreground} guibg={background_alt}
hi! BufferVisibleMod guifg={yellow} guibg={background_alt}
hi! BufferVisibleSign guifg={foreground} guibg={background_alt}
hi! BufferVisibleTarget guifg={red} guibg={background_alt} gui=bold
hi! BufferInactive guifg={foreground_invisible} guibg={background_alt}
hi! BufferInactiveIndex guifg={foreground_invisible} guibg={background_alt}
hi! BufferInactiveMod guifg={yellow} guibg={background_alt}
hi! BufferInactiveSign guifg={foreground_invisible} guibg={background_alt}
hi! BufferInactiveTarget guifg={red} guibg={background_alt} gui=bold

" =============================================
" Plugin: Todo comments
" =============================================
" PERF
hi! TodoBgPERF guifg={background} guibg={purple}
hi! TodoFgPERF guifg={purple}
hi! TodoSignPERF guifg={purple}

" HACK
hi! TodoBgHACK guifg={background} guibg={orange}
hi! TodoFgHACK guifg={orange}
hi! TodoSignHACK guifg={orange}

" TODO
hi! TodoBgTODO guifg={background} guibg={blue}
hi! TodoFgTODO guifg={blue}
hi! TodoSignTODO guifg={blue}

" NOTE
hi! TodoBgNOTE guifg={background} guibg={green}
hi! TodoFgNOTE guifg={green} gui=bold
hi! TodoSignNOTE guifg={green}

" FIX
hi! TodoBgFIX guifg={background} guibg={red}
hi! TodoFgFIX guifg={red} gui=bold
hi! TodoSignFIX guifg={red}

" WARN
hi! TodoBgWARN guifg={background} guibg={yellow}
hi! TodoFgWARN guifg={yellow}
hi! TodoSignWARN guifg={yellow}

" TEST
hi! TodoBgTEST guifg={background} guibg={purple}
hi! TodoFgTEST guifg={purple}
hi! TodoSignTEST guifg={purple}
