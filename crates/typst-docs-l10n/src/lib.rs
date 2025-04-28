//! Localization for typst-docs.

use serde::{Deserialize, Serialize};
use typst::ecow::EcoString;
use typst_docs::{
    CategoryItem, CategoryModel, FuncModel, GroupModel, Html, OutlineItem, ShorthandsModel,
    SymbolModel, SymbolsModel, TypeModel,
};

pub mod generate;
pub mod resolve;
pub mod translate;

/// Details about a documentation page and its children.
#[derive(Debug, Serialize, Deserialize)]
pub struct PageMdModel {
    /// The route to the page.
    pub route: EcoString,
    /// The title of the page.
    pub title: EcoString,
    /// The description of the page.
    pub description: EcoString,
    /// The part of the page.
    pub part: Option<EcoString>,
    /// The outline of the page.
    pub outline: Vec<OutlineMdItem>,
    /// The body of the page.
    pub body: BodyMdModel,
    /// The children of the page.
    pub children: Vec<Self>,
}

impl From<typst_docs::PageModel> for PageMdModel {
    fn from(page: typst_docs::PageModel) -> Self {
        Self {
            route: page.route,
            title: page.title,
            description: page.description,
            part: page.part.map(Into::into),
            outline: page.outline.into_iter().map(Into::into).collect(),
            body: page.body.into(),
            children: page.children.into_iter().map(Self::from).collect(),
        }
    }
}

/// An element in the "On This Page" outline.
#[derive(Debug, Serialize, Deserialize)]
pub struct OutlineMdItem {
    /// The ID of the item.
    pub id: EcoString,
    /// The name of the item.
    pub name: EcoString,
    /// The children of the item.
    pub children: Vec<Self>,
}

impl From<OutlineItem> for OutlineMdItem {
    fn from(item: OutlineItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            children: item.children.into_iter().map(Into::into).collect(),
        }
    }
}

/// The body of a documentation page.
#[derive(Debug, Serialize, Deserialize)]
pub enum BodyMdModel {
    /// An HTML or Markdown ready to be rendered.
    Html(HtmlMd),
    /// A category of functions.
    Category(CategoryMdModel),
    /// Details about a function.
    Func(FuncMdModel),
    /// A group of functions.
    Group(GroupMdModel),
    /// Details about a type.
    Type(TypeMdModel),
    /// A collection of symbols.
    Symbols(SymbolsMdModel),
    /// A list of packages.
    Packages(HtmlMd),
}

impl From<typst_docs::BodyModel> for BodyMdModel {
    fn from(body: typst_docs::BodyModel) -> Self {
        match body {
            typst_docs::BodyModel::Html(html) => BodyMdModel::Html(html.into()),
            typst_docs::BodyModel::Category(category) => BodyMdModel::Category(category.into()),
            typst_docs::BodyModel::Func(func) => BodyMdModel::Func(func.into()),
            typst_docs::BodyModel::Group(group) => BodyMdModel::Group(group.into()),
            typst_docs::BodyModel::Type(type_) => BodyMdModel::Type(type_.into()),
            typst_docs::BodyModel::Symbols(symbols) => BodyMdModel::Symbols(symbols.into()),
            typst_docs::BodyModel::Packages(html) => BodyMdModel::Packages(html.into()),
        }
    }
}

/// Details about a function.
#[derive(Debug, Serialize, Deserialize)]
pub struct FuncMdModel {
    /// The path to the function.
    pub path: Vec<EcoString>,
    /// The name of the function.
    pub name: EcoString,
    /// The title of the function.
    pub title: EcoString,
    /// The keywords of the function.
    pub keywords: Vec<EcoString>,
    /// A one-liner description of the function.
    pub oneliner: EcoString,
    /// Whether the function is an element.
    pub element: bool,
    /// Whether the function is contextual.
    pub contextual: bool,
    /// The deprecation message.
    pub deprecation: Option<EcoString>,
    /// The details of the function.
    pub details: HtmlMd,
    /// This example is only for nested function models. Others can have
    /// their example directly in their details.
    pub example: Option<HtmlMd>,
    /// Whether the function is a method.
    #[serde(rename = "self")]
    pub self_: bool,
    /// The parameters of the function.
    pub params: Vec<ParamMdModel>,
    /// The return types of the function.
    pub returns: Vec<EcoString>,
    /// The scope of the function.
    pub scope: Vec<FuncMdModel>,
}

impl From<FuncModel> for FuncMdModel {
    fn from(func: FuncModel) -> Self {
        Self {
            path: func.path,
            name: func.name,
            title: func.title.into(),
            keywords: func.keywords.iter().copied().map(Into::into).collect(),
            oneliner: func.oneliner.into(),
            element: func.element,
            contextual: func.contextual,
            deprecation: func.deprecation.map(Into::into),
            details: func.details.into(),
            example: func.example.map(Into::into),
            self_: func.self_,
            params: func.params.into_iter().map(Into::into).collect(),
            returns: func.returns.into_iter().map(Into::into).collect(),
            scope: func.scope.into_iter().map(Into::into).collect(),
        }
    }
}

/// Details about a function parameter.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParamMdModel {
    /// The name of the parameter.
    pub name: EcoString,
    /// The details of the parameter.
    pub details: HtmlMd,
    /// An example of the parameter.
    pub example: Option<HtmlMd>,
    /// The types of the parameter.
    pub types: Vec<EcoString>,
    /// The strings that can be passed as the parameter.
    pub strings: Vec<StrParamMd>,
    /// The default value of the parameter.
    pub default: Option<HtmlMd>,
    /// Whether the parameter is positional.
    pub positional: bool,
    /// Whether the parameter is named.
    pub named: bool,
    /// Whether the parameter is required.
    pub required: bool,
    /// Whether the parameter is variadic.
    pub variadic: bool,
    /// Whether the parameter is settable.
    pub settable: bool,
}

impl From<typst_docs::ParamModel> for ParamMdModel {
    fn from(param: typst_docs::ParamModel) -> Self {
        Self {
            name: param.name.into(),
            details: param.details.into(),
            example: param.example.map(Into::into),
            types: param.types.into_iter().map(Into::into).collect(),
            strings: param.strings.into_iter().map(Into::into).collect(),
            default: param.default.map(Into::into),
            positional: param.positional,
            named: param.named,
            required: param.required,
            variadic: param.variadic,
            settable: param.settable,
        }
    }
}

/// Details about a category.
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryMdModel {
    /// The name of the category.
    pub name: EcoString,
    /// The title of the category.
    pub title: EcoString,
    /// The details of the category.
    pub details: HtmlMd,
    /// The items in the category.
    pub items: Vec<CategoryMdItem>,
    /// The shorthands in the category.
    pub shorthands: Option<ShorthandsMdModel>,
}

impl From<CategoryModel> for CategoryMdModel {
    fn from(category: CategoryModel) -> Self {
        Self {
            name: category.name.into(),
            title: category.title,
            details: category.details.into(),
            items: category.items.into_iter().map(Into::into).collect(),
            shorthands: category.shorthands.map(Into::into),
        }
    }
}

/// An HTML or Markdown string.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "kind", content = "content")]
pub enum HtmlMd {
    /// A Markdown string.
    Md(EcoString),
    /// An HTML string.
    Html(EcoString),
}

impl From<Html> for HtmlMd {
    fn from(html: Html) -> Self {
        if !html.as_str().is_empty() && html.md().is_empty() {
            HtmlMd::Html(html.as_str().into())
        } else {
            HtmlMd::Md(html.md().into())
        }
    }
}

/// A specific string that can be passed as an argument.
#[derive(Debug, Serialize, Deserialize)]
pub struct StrParamMd {
    /// The string.
    pub string: EcoString,
    /// The details of the string.
    pub details: HtmlMd,
}

impl From<typst_docs::StrParam> for StrParamMd {
    fn from(param: typst_docs::StrParam) -> Self {
        Self {
            string: param.string,
            details: param.details.into(),
        }
    }
}

/// Details about a group of functions.
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMdModel {
    /// The name of the group.
    pub name: EcoString,
    /// The title of the group.
    pub title: EcoString,
    /// The details of the group.
    pub details: HtmlMd,
    /// The functions in the group.
    pub functions: Vec<FuncMdModel>,
}

impl From<GroupModel> for GroupMdModel {
    fn from(group: GroupModel) -> Self {
        Self {
            name: group.name,
            title: group.title,
            details: group.details.into(),
            functions: group.functions.into_iter().map(FuncMdModel::from).collect(),
        }
    }
}

/// Details about a type.
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeMdModel {
    /// The name of the type.
    pub name: EcoString,
    /// The title of the type.
    pub title: EcoString,
    /// The keywords of the type.
    pub keywords: Vec<EcoString>,
    /// A one-liner description of the type.
    pub oneliner: EcoString,
    /// The details of the type.
    pub details: HtmlMd,
    /// The constructor of the type.
    pub constructor: Option<FuncMdModel>,
    /// The scope of the type.
    pub scope: Vec<FuncMdModel>,
}

impl From<TypeModel> for TypeMdModel {
    fn from(type_: TypeModel) -> Self {
        Self {
            name: type_.name.into(),
            title: type_.title.into(),
            keywords: type_.keywords.iter().copied().map(Into::into).collect(),
            oneliner: type_.oneliner.into(),

            details: type_.details.into(),
            constructor: type_.constructor.map(FuncMdModel::from),
            scope: type_.scope.into_iter().map(FuncMdModel::from).collect(),
        }
    }
}

/// A collection of symbols.
#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolsMdModel {
    /// The name of the symbols.
    pub name: EcoString,
    /// The title of the symbols.
    pub title: EcoString,
    /// The details of the symbols.
    pub details: HtmlMd,
    /// The list of symbols.
    pub list: Vec<SymbolMdModel>,
}

impl From<SymbolsModel> for SymbolsMdModel {
    fn from(symbols: SymbolsModel) -> Self {
        Self {
            name: symbols.name,
            title: symbols.title,
            details: symbols.details.into(),
            list: symbols.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// Details about a category item.
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryMdItem {
    /// The name of the item.
    pub name: EcoString,
    /// The route to the item.
    pub route: EcoString,
    /// A one-liner description of the item.
    pub oneliner: EcoString,
    /// Whether the item is a code.
    pub code: bool,
}

impl From<CategoryItem> for CategoryMdItem {
    fn from(item: CategoryItem) -> Self {
        Self {
            name: item.name,
            route: item.route,
            oneliner: item.oneliner,
            code: item.code,
        }
    }
}

/// Shorthands listed on a category page.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShorthandsMdModel {
    /// The markup shorthands.
    pub markup: Vec<SymbolMdModel>,
    /// The math shorthands.
    pub math: Vec<SymbolMdModel>,
}

impl From<ShorthandsModel> for ShorthandsMdModel {
    fn from(shorthands: ShorthandsModel) -> Self {
        Self {
            markup: shorthands.markup.into_iter().map(Into::into).collect(),
            math: shorthands.math.into_iter().map(Into::into).collect(),
        }
    }
}

/// Details about a symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolMdModel {
    /// The name of the symbol.
    pub name: EcoString,
    /// The codepoint of the symbol.
    pub codepoint: u32,
    /// Whether the symbol is an accent.
    pub accent: bool,
    /// The alternates of the symbol.
    pub alternates: Vec<EcoString>,
    /// The markup shorthand of the symbol.
    pub markup_shorthand: Option<EcoString>,
    /// The math shorthand of the symbol.
    pub math_shorthand: Option<EcoString>,
    /// The math class of the symbol.
    pub math_class: Option<EcoString>,
    /// The deprecation message.
    pub deprecation: Option<EcoString>,
}

impl From<SymbolModel> for SymbolMdModel {
    fn from(symbol: SymbolModel) -> Self {
        Self {
            name: symbol.name,
            codepoint: symbol.codepoint,
            accent: symbol.accent,
            alternates: symbol.alternates,
            markup_shorthand: symbol.markup_shorthand.map(Into::into),
            math_shorthand: symbol.math_shorthand.map(Into::into),
            math_class: symbol.math_class.map(Into::into),
            deprecation: symbol.deprecation.map(Into::into),
        }
    }
}

/// Convert a path to a dot path.
fn to_dot_path(path: &str) -> String {
    path.trim_matches('/').replace("/", ".")
}
