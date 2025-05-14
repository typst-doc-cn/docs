#set heading(numbering: "1.")
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

== Bad reference `<guides.page-setup-guide.bodycolumns>` <guides.page-setup-guide.bodycolumns>

== Bad reference `<guides.table-guide.bodycolumn-sizes>` <guides.table-guide.bodycolumn-sizes>

== Bad reference `<guides.table-guide.bodystrokes>` <guides.table-guide.bodystrokes>

== Bad reference `<guides.table-guide.bodyfills>` <guides.table-guide.bodyfills>

== Bad reference `<guides.table-guide.bodystroke-functions>` <guides.table-guide.bodystroke-functions>

== Bad reference `<guides.table-guide.bodyimporting-data>` <guides.table-guide.bodyimporting-data>

== Bad reference `<guides.table-guide.bodyindividual-lines>` <guides.table-guide.bodyindividual-lines>

== Bad reference `<guides.table-guide.bodyalignment>` <guides.table-guide.bodyalignment>

== Bad reference `<reference.syntax.bodyescapes>` <reference.syntax.bodyescapes>

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
