//! Get the translation pairs for the documentation.

use crate::*;

/// The translation pairs.
type TranslationPairs = Vec<(String, String)>;

/// Check a page for translations.
pub fn check_page(page: PageMdModel, translations: &mut TranslationPairs) {
    for child in page.children {
        check_page(child, translations);
    }

    if page.route.contains("changelog") {
        return;
    }

    let k = to_dot_path(&page.route);
    let k = if k.is_empty() { "index".to_owned() } else { k };

    translations.push((format!("{k}.title"), page.title.into()));
    translations.push((format!("{k}.description"), page.description.into()));

    if let Some(part) = page.part {
        translations.push((format!("{part}.part"), part.into()));
    }

    // check_outline(page.outline, &k, translations);
    check_body(page.body, &k, translations);
}

/// Check the body for translations.
fn check_body(body: BodyMdModel, k: &str, translations: &mut TranslationPairs) {
    match body {
        BodyMdModel::Html(html) => {
            let k = format!("{k}.body");
            check_html(html, &k, translations);
        }
        BodyMdModel::Category(category) => {
            check_category(category, k, translations);
        }
        BodyMdModel::Func(func) => {
            check_func(func, k, translations);
        }
        BodyMdModel::Group(group) => {
            check_group(group, k, translations);
        }
        BodyMdModel::Type(type_) => {
            check_type(type_, k, translations);
        }
        BodyMdModel::Symbols(symbols) => {
            check_symbols(symbols, k, translations);
        }
        BodyMdModel::Packages(html) => {
            let k = format!("{k}.packages");
            check_html(html, &k, translations);
        }
    }
}

/// Check the category for translations.
fn check_category(category: CategoryMdModel, k: &str, translations: &mut TranslationPairs) {
    let k = format!("{k}.{}", category.name);

    translations.push((format!("{k}.title"), category.title.into()));
    {
        let k = format!("{k}.details");
        check_html(category.details, &k, translations);
    }

    for item in category.items {
        let k = to_dot_path(&item.route);

        // translations.push((format!("{k}.name"), item.name.into()));
        translations.push((format!("{k}.oneliner"), item.oneliner.into()));
    }

    if let Some(shorthands) = category.shorthands {
        check_shorthands(shorthands, &k, translations);
    }
}

/// Check the shorthands for translations.
fn check_shorthands(shorthands: ShorthandsMdModel, k: &str, translations: &mut TranslationPairs) {
    let k = format!("{k}.shorthands");

    for symbol in shorthands.markup {
        let k = format!("{k}.markup");
        check_symbol(symbol, &k, translations);
    }

    for symbol in shorthands.math {
        let k = format!("{k}.math");
        check_symbol(symbol, &k, translations);
    }
}

/// Check the function for translations.
fn check_func(func: FuncMdModel, k: &str, translations: &mut TranslationPairs) {
    let k = format!("{k}.{}", func.name);

    translations.push((format!("{k}.title"), func.title.into()));
    translations.push((format!("{k}.oneliner"), func.oneliner.into()));
    if let Some(deprecation) = func.deprecation {
        translations.push((format!("{k}.deprecation"), deprecation.into()));
    }
    {
        let k = format!("{k}.details");
        check_html(func.details, &k, translations);
    }

    for param in func.params {
        check_param(param, &k, translations);
    }
    for scope in func.scope {
        check_func(scope, &k, translations);
    }
}

/// Check the parameter for translations.
fn check_param(param: ParamMdModel, k: &str, translations: &mut TranslationPairs) {
    let k = format!("{k}.{}", param.name);

    {
        let k = format!("{k}.details");
        check_html(param.details, &k, translations);
    }
}

/// Check the symbol for translations.
fn check_symbol(symbol: SymbolMdModel, k: &str, translations: &mut TranslationPairs) {
    let k = format!("{k}.{}", symbol.codepoint);

    if let Some(deprecation) = symbol.deprecation {
        translations.push((format!("{k}.deprecation"), deprecation.into()));
    }
}

/// Check the group for translations.
fn check_group(group: GroupMdModel, k: &str, translations: &mut TranslationPairs) {
    let k = format!("{k}.{}", group.name);

    translations.push((format!("{k}.title"), group.title.into()));
    {
        let k = format!("{k}.details");
        check_html(group.details, &k, translations);
    }

    for func in group.functions {
        check_func(func, &k, translations);
    }
}

/// Check the type for translations.
fn check_type(type_: TypeMdModel, k: &str, translations: &mut TranslationPairs) {
    let k = format!("{k}.{}", type_.name);

    translations.push((format!("{k}.title"), type_.title.into()));
    translations.push((format!("{k}.oneliner"), type_.oneliner.into()));
    {
        let k = format!("{k}.details");
        check_html(type_.details, &k, translations);
    }

    if let Some(constructor) = type_.constructor {
        check_func(constructor, &k, translations);
    }
    for scope in type_.scope {
        check_func(scope, &k, translations);
    }
}

/// Check the symbols for translations.
fn check_symbols(symbols: SymbolsMdModel, k: &str, translations: &mut TranslationPairs) {
    let k = format!("{k}.{}", symbols.name);

    translations.push((format!("{k}.title"), symbols.title.into()));
    {
        let k = format!("{k}.details");
        check_html(symbols.details, &k, translations);
    }

    for symbol in symbols.list {
        check_symbol(symbol, &k, translations);
    }
}

/// Check the children for translations.
fn check_html(html: HtmlMd, k: &str, translations: &mut TranslationPairs) {
    match html {
        HtmlMd::Html(text) => {
            translations.push((k.into(), text.into()));
        }
        HtmlMd::Md(code) => {
            translations.push((k.into(), code.into()));
        }
    }
}
