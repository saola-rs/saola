#[cfg(all(feature = "lsp", any(feature = "postgresql", feature = "cockroachdb", feature = "mssql")))]
use lsp_types::*;

#[cfg(feature = "postgresql")]
pub(crate) fn extensions_completion(
    #[cfg(feature = "lsp")] completion_list: &mut crate::datamodel_connector::CompletionList,
    #[cfg(not(feature = "lsp"))] _completion_list: &mut crate::datamodel_connector::CompletionList,
) {
    #[cfg(feature = "lsp")]
    completion_list.items.push(CompletionItem {
        label: "extensions".to_owned(),
        insert_text: Some("extensions = [$0]".to_owned()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        kind: Some(CompletionItemKind::FIELD),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: crate::datamodel_connector::format_completion_docs(
                r#"extensions = [pg_trgm, postgis(version: "2.1")]"#,
                r#"Enable PostgreSQL extensions. [Learn more](https://pris.ly/d/postgresql-extensions)"#,
                None,
            ),
        })),
        ..Default::default()
    })
}

#[cfg(any(feature = "postgresql", feature = "cockroachdb", feature = "mssql"))]
pub(crate) fn schemas_completion(
    #[cfg(feature = "lsp")] completion_list: &mut crate::datamodel_connector::CompletionList,
    #[cfg(not(feature = "lsp"))] _completion_list: &mut crate::datamodel_connector::CompletionList,
) {
    #[cfg(feature = "lsp")]
    completion_list.items.push(CompletionItem {
        label: "schemas".to_owned(),
        insert_text: Some(r#"schemas = [$0]"#.to_owned()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        kind: Some(CompletionItemKind::FIELD),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: crate::datamodel_connector::format_completion_docs(
                r#"schemas = ["foo", "bar", "baz"]"#,
                "The list of database schemas. [Learn More](https://pris.ly/d/multi-schema-configuration)",
                None,
            ),
        })),
        // detail: Some("schemas".to_owned()),
        ..Default::default()
    });
}
