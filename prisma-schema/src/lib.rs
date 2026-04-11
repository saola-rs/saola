/// Simple wrapper around the official PSL parser
/// Provides utilities for parsing schema.prisma files using the official parser

pub use parser_database::ParserDatabase;
pub use schema_ast::ast::{SchemaAst, Top, TopId};

pub mod parser;

pub use parser::{parse_schema_file, parse_schema_str};
