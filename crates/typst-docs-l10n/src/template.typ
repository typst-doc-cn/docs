#set heading(numbering: "1.")
#set page(numbering: "1")
#set par(justify: true)
#set text(
  font: (
    "Libertinus Serif",
    "Source Han Serif SC",
  ),
  lang: "zh",
  region: "cn",
)

#align(center)[
  #set text(size: 36pt)
  Typst官方文档翻译
]


#pagebreak()

#outline()

#pagebreak()

#show raw.where(lang: "example"): it => {
  raw(
    lang: "typ",
    align: it.align,
    block: it.block,
    syntaxes: it.syntaxes,
    tab-size: it.tab-size,
    theme: it.theme,
    it.text,
  )
}
