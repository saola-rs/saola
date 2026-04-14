mod codegen;

use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "prisma-cli")]
#[command(about = "Prisma Rust Client Generator", long_about = None)]
struct Cli {
    /// Path to the schema.prisma file
    #[arg(short, long, default_value = "schema.prisma")]
    schema: String,

    /// Output directory for the generated Rust code
    #[arg(short, long, default_value = "src/prisma.rs")]
    output: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("Loading schema from: {}", cli.schema);

    let db = match prisma_schema::parser::parse_schema_file(&cli.schema) {
        Ok(db) => db,
        Err(e) => anyhow::bail!("{}", e),
    };

    println!("Schema validation successful.");
    println!("Found {} models.", db.walk_models().count());

    // Code Generation
    let token_stream = codegen::generate_client(&db);
    let generated_code = token_stream.to_string();

    // Format the generated code
    let parsed_file = syn::parse_file(&generated_code)?;
    let formatted_code = prettyplease::unparse(&parsed_file);

    // Ensure the output directory exists
    if let Some(parent) = Path::new(&cli.output).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&cli.output, formatted_code)?;
    println!("Successfully generated code to: {}", cli.output);

    Ok(())
}
