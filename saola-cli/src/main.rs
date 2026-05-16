use clap::Parser;
use saola_codegen::Generator;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Saola ORM CLI for code generation")]
struct Cli {
    /// Path to the Prisma schema file or directory containing schema files
    #[arg(short, long, default_value = "schema.prisma")]
    schema: PathBuf,

    /// Output directory for generated code
    #[arg(short, long, default_value = "src/saola")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("🚀 Generating Saola client from {:?}...", cli.schema);

    let generator = Generator::new(cli.schema)?;
    generator.generate(cli.output)?;

    println!("✅ Done! Code generated in the output directory.");
    Ok(())
}
