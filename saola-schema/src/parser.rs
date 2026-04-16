use parser_database::ParserDatabase;
use psl::SourceFile;
use psl_core::{builtin_connectors, parser_database::NoExtensionTypes};

/// Parse a schema from a file path
pub fn parse_schema_file(path: &str) -> Result<ParserDatabase, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Cannot read schema file '{}': {}", path, e))?;
    parse_schema_str(&content)
}

/// Parse a schema from a string
pub fn parse_schema_str(schema_str: &str) -> Result<ParserDatabase, String> {
    let file = SourceFile::from(schema_str);
    let connectors = builtin_connectors::BUILTIN_CONNECTORS;
    let ext_types = NoExtensionTypes;

    let validated = psl_core::validate(file, connectors, &ext_types);

    if !validated.diagnostics.errors().is_empty() {
        let err_msg = validated
            .diagnostics
            .errors()
            .iter()
            .map(|e| e.message())
            .collect::<Vec<_>>()
            .join("\n");
        return Err(format!("Schema validation failed: {}", err_msg));
    }

    Ok(validated.db)
}
