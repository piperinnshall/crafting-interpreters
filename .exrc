if &cp | set nocp | endif
let s:cpo_save=&cpo
set cpo&vim
inoremap <silent> <Plug>(-fzf-complete-finish) l
imap <C-G>S <Plug>ISurround
imap <C-G>s <Plug>Isurround
imap <C-S> <Plug>Isurround
inoremap <silent> <Plug>(fzf-maps-i) :call fzf#vim#maps('i', 0)
inoremap <expr> <Plug>(fzf-complete-buffer-line) fzf#vim#complete#buffer_line()
inoremap <expr> <Plug>(fzf-complete-line) fzf#vim#complete#line()
inoremap <expr> <Plug>(fzf-complete-file-ag) fzf#vim#complete#path('ag -l -g ""')
inoremap <expr> <Plug>(fzf-complete-file) fzf#vim#complete#path("find . -path '*/\.*' -prune -o -type f -print -o -type l -print | sed 's:^..::'")
inoremap <expr> <Plug>(fzf-complete-path) fzf#vim#complete#path("find . -path '*/\.*' -prune -o -print | sed '1d;s:^..::'")
inoremap <expr> <Plug>(fzf-complete-word) fzf#vim#complete#word()
map! <D-v> *
nnoremap  <Cmd>cclose
nnoremap  <Cmd>Rg
nnoremap  <Cmd>Files
nnoremap  <Cmd>nohlsearch
nnoremap  u <Cmd>UndotreeToggle
nnoremap  ct <Cmd>CTags
nnoremap  e <Cmd>Ex
nnoremap  y "+y
vnoremap  y "+y
xmap S <Plug>VSurround
nmap cS <Plug>CSurround
nmap cs <Plug>Csurround
nmap ds <Plug>Dsurround
xmap gS <Plug>VgSurround
nmap gcu <Plug>Commentary<Plug>Commentary
omap gc <Plug>Commentary
nmap gc <Plug>Commentary
xmap gc <Plug>Commentary
xmap gx <Plug>(open-word-under-cursor)
nmap gx <Plug>(open-word-under-cursor)
snoremap gc <Plug>Commentary
nmap gcc <Plug>CommentaryLine
nnoremap g= <Cmd>Format
nmap ySS <Plug>YSsurround
nmap ySs <Plug>YSsurround
nmap yss <Plug>Yssurround
nmap yS <Plug>YSurround
nmap ys <Plug>Ysurround
nnoremap z= <Cmd>Spell
nnoremap <silent> <Plug>(-fzf-complete-finish) a
nnoremap <Plug>(-fzf-:) :
nnoremap <Plug>(-fzf-/) /
nnoremap <Plug>(-fzf-vim-do) :execute g:__fzf_command
nnoremap <silent> <Plug>SurroundRepeat .
nmap <silent> <Plug>CommentaryUndo :echoerr "Change your <Plug>CommentaryUndo map to <Plug>Commentary<Plug>Commentary"
onoremap <silent> <Plug>(fzf-maps-o) :call fzf#vim#maps('o', 0)
xnoremap <silent> <Plug>(fzf-maps-x) :call fzf#vim#maps('x', 0)
nnoremap <silent> <Plug>(fzf-maps-n) :call fzf#vim#maps('n', 0)
tnoremap <silent> <Plug>(fzf-normal) 
tnoremap <silent> <Plug>(fzf-insert) i
nnoremap <silent> <Plug>(fzf-normal) <Nop>
nnoremap <silent> <Plug>(fzf-insert) i
xnoremap <Plug>(open-word-under-cursor) <ScriptCmd>vim9.Open(getregion(getpos('v'), getpos('.'), { type: mode() })->join())
nnoremap <Plug>(open-word-under-cursor) <ScriptCmd>vim9.Open(GetWordUnderCursor())
nnoremap <C-G> <Cmd>Rg
nnoremap <C-P> <Cmd>Files
nnoremap <C-C> <Cmd>cclose
vmap <BS> "-d
vmap <D-x> "*d
vmap <D-c> "*y
vmap <D-v> "-d"*P
nmap <D-v> "*P
imap S <Plug>ISurround
imap s <Plug>Isurround
imap  <Plug>Isurround
iabbr @@ test
iabbr main@ public class Main {public static void main(String args) {}}kk
let &cpo=s:cpo_save
unlet s:cpo_save
set autowrite
set backspace=2
set expandtab
set exrc
set fileencodings=ucs-bom,utf-8,default,latin1
set fillchars=eob:\ ,fold:\ ,foldopen:â”‚,foldsep:â”‚,foldclose:â€º
set helplang=en
set hlsearch
set ignorecase
set incsearch
set laststatus=2
set modelines=0
set runtimepath=~/.vim,~/.vim/pack/plugins/start/vim-surround,~/.vim/pack/plugins/start/vim-commentary,~/.vim/pack/plugins/start/undotree,~/.vim/pack/plugins/start/fzf-vim,~/.vim/pack/plugins/start/fzf,~/.vim/pack/plugins/start/everforest,/usr/share/vim/vimfiles,/usr/share/vim/vim91,/usr/share/vim/vim91/pack/dist/opt/netrw,/usr/share/vim/vimfiles/after,~/.vim/after
set scrolloff=8
set secure
set shell=/etc/profiles/per-user/piperinnshall/bin/bash
set shiftwidth=2
set noshowmode
set smartcase
set smartindent
set statusline=\ \ \ %f\ %l:%c\ %m
set noswapfile
set tabstop=2
set termguicolors
set ttimeoutlen=0
set undodir=~/.vim/undodir
set undofile
set updatetime=50
set viewoptions=folds
set window=0
" vim: set ft=vim :
