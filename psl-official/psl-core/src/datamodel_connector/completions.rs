use std::collections::HashMap;

#[cfg(not(feature = "lsp"))]
#[derive(Debug, Default, Clone)]
pub struct CompletionList {
    pub is_incomplete: bool,
    pub items: Vec<CompletionItem>,
}

#[cfg(not(feature = "lsp"))]
#[derive(Debug, Default, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: Option<CompletionItemKind>,
    pub detail: Option<String>,
    pub documentation: Option<Documentation>,
    pub insert_text: Option<String>,
    pub insert_text_format: Option<InsertTextFormat>,
}

#[cfg(not(feature = "lsp"))]
#[derive(Debug, Clone)]
pub enum Documentation {
    MarkupContent(MarkupContent),
}

#[cfg(not(feature = "lsp"))]
#[derive(Debug, Clone)]
pub struct MarkupContent {
    pub kind: MarkupKind,
    pub value: String,
}

#[cfg(not(feature = "lsp"))]
#[derive(Debug, Clone, Copy)]
pub enum MarkupKind {
    Markdown,
    PlainText,
}

#[cfg(not(feature = "lsp"))]
#[derive(Debug, Clone, Copy)]
pub enum CompletionItemKind {
    PROPERTY,
    ENUM,
    EnumMember,
    Constant,
    FIELD,
    FUNCTION,
}

#[cfg(not(feature = "lsp"))]
impl CompletionItemKind {
    pub const FIELD: Self = Self::FIELD;
    pub const ENUM: Self = Self::ENUM;
    pub const FUNCTION: Self = Self::FUNCTION;
    pub const PROPERTY: Self = Self::PROPERTY;
}

#[cfg(not(feature = "lsp"))]
#[derive(Debug, Clone, Copy)]
pub enum InsertTextFormat {
    PlainText,
    Snippet,
    SNIPPET,
}

#[cfg(not(feature = "lsp"))]
impl InsertTextFormat {
    pub const SNIPPET: Self = Self::SNIPPET;
}

/// Formats the documentation for a completion.
/// example: How the completion is expected to be used.
///
/// # Example
///
/// ```
/// use psl_core::datamodel_connector::format_completion_docs;
///
/// let doc = format_completion_docs(
///     r#"relationMode = "foreignKeys" | "prisma""#,
///     r#"Sets the global relation mode for relations."#,
///     None,
/// );
///
/// assert_eq!(
///     "```prisma\nrelationMode = \"foreignKeys\" | \"prisma\"\n```\n___\nSets the global relation mode for relations.\n\n",
///     &doc
/// );
/// ```
pub fn format_completion_docs(example: &str, description: &str, params: Option<HashMap<&str, &str>>) -> String {
    let param_docs: String = match params {
        Some(params) => params
            .into_iter()
            .map(|(param_label, param_doc)| format!("_@param_ {param_label} {param_doc}"))
            .collect::<Vec<String>>()
            .join("\n"),
        None => Default::default(),
    };

    format!("```prisma\n{example}\n```\n___\n{description}\n\n{param_docs}")
}
