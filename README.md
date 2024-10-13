# TeX入門

高校の冊子用に作った物です。圧倒的に終わってる文章力で作ったのでまあ多少はね？

VS Code用の設定をコピペしたい場合は以下をそのままはっつけて下さい。
```
"latex-workshop.latex.tools": [
    {
        "command": "mkdir",
        "args": [
            "-p",
            "output",
        ],
        "name": "tool_mkdir"
    },
    {
        "command": "ptex2pdf",
        "args": [
            "-l",
            "-ot",
            "-kanji=utf8 -synctex=1",
            "%DOC%",
            "-output-directory",
            "output",
        ],
        "name": "tool_ptex2pdf"
    },
    {
        "command": "pbibtex",
        "args": [
            "output/%DOCFILE%",
            "-kanji=utf8"
        ],
        "name": "tool_pbibtex"
    }
],
"latex-workshop.latex.recipes": [
    {
        "name": "tex2pdf",
        "tools": [
            "tool_mkdir",
            "tool_ptex2pdf",
        ],
    },
    {
        "name": "tex2pdf with bib",
        "tools": [
            "tool_ptex2pdf",
            "tool_pbibtex",
            "tool_ptex2pdf",
            "tool_ptex2pdf"
        ]
    }
],
"latex-workshop.view.pdf.viewer": "tab",
"latex-workshop.intellisense.package.enabled": true,
"[latex]": {
    "editor.wordWrap": "on",
    "editor.wordSeparators": "./\\()\"'-:,.;<>~!@#$%^&*|+=[]{}`~?。．、，（）「」『』［］｛｝《》てにをはがのともへでや ",
    "editor.tabSize": 2,
    "editor.insertSpaces": true,
    "editor.detectIndentation": false,
    "editor.suggestSelection": "recentlyUsedByPrefix",
    "editor.suggest.snippetsPreventQuickSuggestions": false,
    "editor.quickSuggestions": {
        "other": true,
        "comments": false,
        "strings": false
    },
    "editor.bracketPairColorization.enabled": true,
    "editor.unicodeHighlight.invisibleCharacters": true,
    "editor.unicodeHighlight.allowedCharacters": {
        "，": true,
        "．": true,
        "！": true,
        "？": true,
        "［": true,
        "］": true,
        "｛": true,
        "｝": true,
        "＜": true,
        "＞": true,
    },
    "editor.stickyScroll.enabled": true,
},
"latex-workshop.latex.outDir": "%DIR%/output",
"latex-workshop.message.error.show": false,
"latex-workshop.message.warning.show": false,
"latex-workshop.formatting.latex": "latexindent",
```
