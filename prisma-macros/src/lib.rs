extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;

mod codegen;

/// Generate the Prisma client code from schema.prisma
///
/// Usage:
/// ```rust
/// prisma_macros::prisma_client!();
/// ```
#[proc_macro]
pub fn prisma_client(_input: TokenStream) -> TokenStream {
    // 1. Find schema.prisma in the manifest directory
    let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(d) => d,
        Err(_) => {
            let error = "CARGO_MANIFEST_DIR not set - macro must be used in a crate with Cargo.toml";
            return syn::Error::new(proc_macro2::Span::call_site(), error)
                .to_compile_error()
                .into();
        }
    };
    let schema_path = format!("{}/schema.prisma", manifest_dir);

    // 2. Parse the schema using the official PSL parser
    let db = match prisma_schema::parser::parse_schema_file(&schema_path) {
        Ok(db) => {
            eprintln!("[PRISMA] Parsed schema from: {}", schema_path);
            db
        }
        Err(e) => {
            let error_msg = format!("Failed to parse schema.prisma at '{}': {}", schema_path, e);
            eprintln!("[PRISMA ERROR] {}", error_msg);
            return syn::Error::new(proc_macro2::Span::call_site(), error_msg)
                .to_compile_error()
                .into();
        }
    };

    // 3. Generate code from the parsed schema
    let generated = codegen::generate_client(&db);
    eprintln!("[PRISMA] Code generation complete");

    // 4. Output as token stream
    TokenStream::from(generated)
}

