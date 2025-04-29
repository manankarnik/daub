" {color0}
" {color1} 
" {color2} 
" {color3} 
" {color4} 
" {color5} 
" {color6} 
" {color7}
" {color8}
" {color9}
" {color10}
" {color11}
" {color12}
" {color13}
" {color14}
" {color15}
" {foreground}
" {background}
" {cursor}
hi clear
set background={mode}
if exists("syntax on")
    syntax reset
endif
let g:colors_name="daub"

hi! Normal guifg={foreground} guibg={background}
hi! Comment guifg={color8} cterm=italic gui=italic
hi! Constant guifg={color3}
hi! String guifg={color2}
hi! Identifier guifg={color3}
hi! Function guifg={color4}
hi! Statement guifg={color5}
hi! PreProc guifg={color2}
hi! Type guifg={color6}
hi! Special guifg={color5}
hi! Underlined guifg={color4}
hi! Error guifg={color1}
hi! Todo guifg={color2}

hi! link Delimiter Normal  
hi! link Operator Normal  
hi! link Title Normal

hi! @variable guifg={foreground}
hi! @variable.builtin guifg={color9}
hi! @keyword guifg={color5} cterm=italic gui=italic

hi! link @keyword.conditional Statement
hi! link @label Function
