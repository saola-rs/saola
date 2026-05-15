pub mod builder_gen;
pub mod model_analysis;
pub mod model_gen;
pub mod query_gen;
pub mod utils;
pub mod wrapper_gen;

use anyhow::Context;
use std::fs;
use std::io::Write;
use std::path::Path;

pub struct Generator {
    schema: psl::ValidatedSchema,
    schema_content: String,
}

impl Generator {
    pub fn new(schema_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let schema_content = fs::read_to_string(&schema_path)?;
        let source_file = psl::SourceFile::from(schema_content.as_str());
        let schema = psl::validate(source_file, &psl::parser_database::NoExtensionTypes);

        if !schema.diagnostics.errors().is_empty() {
            anyhow::bail!("Schema validation failed: {:?}", schema.diagnostics.errors());
        }

        Ok(Self { schema, schema_content })
    }

    pub fn generate(&self, output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
        let output_dir = output_dir.as_ref();
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        // Collect metadata once
        let mut model_metadata_map = std::collections::HashMap::new();
        let db = &self.schema.db;

        for walker in db.walk_models() {
            let mut fields = Vec::new();
            let relation_link_fields: std::collections::HashSet<_> = walker
                .relation_fields()
                .filter_map(|rf| rf.referencing_fields())
                .flatten()
                .map(|f| f.field_id())
                .collect();

            for field in walker.scalar_fields() {
                let field_type_str = match field.scalar_field_type() {
                    psl::parser_database::ScalarFieldType::BuiltInScalar(psl::parser_database::ScalarType::String) => {
                        "String"
                    }
                    psl::parser_database::ScalarFieldType::BuiltInScalar(psl::parser_database::ScalarType::Int) => {
                        "i32"
                    }
                    psl::parser_database::ScalarFieldType::BuiltInScalar(psl::parser_database::ScalarType::Boolean) => {
                        "bool"
                    }
                    psl::parser_database::ScalarFieldType::Enum(id) => db.walk(id).name(),
                    _ => "String",
                }
                .to_string();

                let mut field_type: syn::Type = syn::parse_str(&field_type_str).unwrap();
                if !field.ast_field().arity.is_required() {
                    field_type = syn::parse_quote! { Option<#field_type> };
                }
                let enum_name: Option<String> = if let as_enum = field.field_type_as_enum()
                    && as_enum.is_some()
                {
                    Some(as_enum.unwrap().name().to_string())
                } else {
                    None
                };

                fields.push(crate::model_analysis::FieldMetadata {
                    rust_name: heck::ToSnakeCase::to_snake_case(field.name()),
                    prisma_name: field.name().to_string(),
                    is_relation: false,
                    is_unique: field.is_unique(),
                    is_id: walker
                        .primary_key()
                        .map(|pk| pk.fields().any(|f| f.field_id() == field.field_id()))
                        .unwrap_or(false),
                    is_optional: !field.ast_field().arity.is_required(),
                    is_list: field.ast_field().arity.is_list(),
                    is_relation_link: relation_link_fields.contains(&field.field_id()),
                    has_default: field.default_value().is_some(),
                    is_updated_at: field.is_updated_at(),
                    opposite_relation_field: None,
                    field_type,
                    enum_name,
                });
            }
            for field in walker.relation_fields() {
                fields.push(crate::model_analysis::FieldMetadata {
                    rust_name: heck::ToSnakeCase::to_snake_case(field.name()),
                    prisma_name: field.name().to_string(),
                    is_relation: true,
                    is_unique: false,
                    is_id: false,
                    is_optional: !field.ast_field().arity.is_required(),
                    is_list: field.ast_field().arity.is_list(),
                    is_relation_link: false,
                    has_default: false,
                    is_updated_at: false,
                    opposite_relation_field: field.opposite_relation_field().map(|f| f.name().to_string()),
                    field_type: syn::parse_str(field.related_model().name()).unwrap(),
                    enum_name: None,
                });
            }
            model_metadata_map.insert(
                walker.name().to_string(),
                crate::model_analysis::ModelMetadata::new(walker.name().to_string(), fields),
            );
        }

        self.generate_mod_rs(output_dir)?;
        self.generate_enums_rs(output_dir)?;

        for walker in db.walk_models() {
            let model_name_str = walker.name();
            let name_snake = heck::ToSnakeCase::to_snake_case(model_name_str);
            let model_dir = output_dir.join(format!("{}_dir", name_snake));
            if !model_dir.exists() {
                fs::create_dir_all(&model_dir)?;
            }

            self.generate_model_mod_rs(&model_dir, model_name_str)?;
            self.generate_model_struct_rs(&model_dir, walker, &model_metadata_map)?;
            self.generate_model_builders_rs(&model_dir, walker, &model_metadata_map)?;
        }

        self.prettify(output_dir)?;

        Ok(())
    }

    fn prettify(&self, output_dir: &Path) -> anyhow::Result<()> {
        let mut files = Vec::new();
        self.collect_rs_files(output_dir, &mut files)?;

        if files.is_empty() {
            return Ok(());
        }

        let status = std::process::Command::new("rustfmt")
            .arg("--edition")
            .arg("2024")
            .args(files)
            .status();

        match status {
            Ok(s) if s.success() => {
                // println!("✅ Code formatted successfully");
            }
            _ => {
                // println!("⚠️ Failed to run rustfmt. Ensure it is installed to prettify generated code.");
            }
        }

        Ok(())
    }

    fn collect_rs_files(&self, dir: &Path, files: &mut Vec<std::path::PathBuf>) -> anyhow::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.collect_rs_files(&path, files)?;
                } else if path.extension().map_or(false, |ext| ext == "rs") {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    fn generate_mod_rs(&self, output_dir: &Path) -> anyhow::Result<()> {
        let mut f = fs::File::create(output_dir.join("mod.rs"))?;

        writeln!(f, "// Generated by Saola CLI. DO NOT EDIT.")?;
        writeln!(f, "pub mod enums;")?;
        writeln!(f, "#[allow(unused_imports)] pub use enums::*;")?;
        writeln!(f, "")?;

        for walker in self.schema.db.walk_models() {
            let name_snake = heck::ToSnakeCase::to_snake_case(walker.name());
            writeln!(f, "mod {}_dir;", name_snake)?;
            writeln!(f, "pub use {}_dir::mod_exports::*;", name_snake)?;
        }

        writeln!(f, "")?;
        writeln!(f, "#[allow(unused_imports)] pub mod prelude {{")?;
        writeln!(f, "    pub use ::saola_core::prelude::*;")?;
        writeln!(f, "}}")?;
        writeln!(f, "")?;
        writeln!(f, "#[allow(unused_imports)] pub mod builders {{")?;
        for walker in self.schema.db.walk_models() {
            let name_snake = heck::ToSnakeCase::to_snake_case(walker.name());
            writeln!(f, "    pub use super::{}_dir::builders::*;", name_snake)?;
        }
        writeln!(f, "}}")?;

        writeln!(f, "")?;
        let datasource = self
            .schema
            .configuration
            .datasources
            .first()
            .context("No datasource found")?;
        let url_tokens = if let Some(env_var) = datasource.url.as_env_var() {
            format!(
                "std::env::var(\"{}\").unwrap_or_else(|_| String::new()).as_str()",
                env_var
            )
        } else {
            format!("\"{}\"", datasource.url.as_literal().unwrap_or(""))
        };

        writeln!(
            f,
            "#[derive(Clone)] pub struct SaolaClient(pub ::saola_core::SaolaClient);"
        )?;
        writeln!(
            f,
            "impl std::ops::Deref for SaolaClient {{ type Target = ::saola_core::SaolaClient; fn deref(&self) -> &Self::Target {{ &self.0 }} }}"
        )?;
        writeln!(f, "")?;

        writeln!(
            f,
            "impl ::saola_core::transaction::QueryExecutorProvider for SaolaClient {{"
        )?;
        writeln!(
            f,
            "    fn executor(&self) -> std::sync::Arc<dyn ::saola_core::query_core::QueryExecutor + Send + Sync> {{ self.0.executor() }}"
        )?;
        writeln!(
            f,
            "    fn query_schema(&self) -> std::sync::Arc<::saola_core::schema::QuerySchema> {{ self.0.query_schema() }}"
        )?;
        writeln!(
            f,
            "    fn tx_id(&self) -> Option<&::saola_core::query_core::TxId> {{ self.0.tx_id() }}"
        )?;
        writeln!(f, "}}")?;
        writeln!(f, "")?;

        writeln!(
            f,
            "#[derive(Clone)] pub struct Transaction(pub ::saola_core::transaction::Transaction);"
        )?;
        writeln!(
            f,
            "impl std::ops::Deref for Transaction {{ type Target = ::saola_core::transaction::Transaction; fn deref(&self) -> &Self::Target {{ &self.0 }} }}"
        )?;
        writeln!(f, "")?;

        writeln!(
            f,
            "impl ::saola_core::transaction::QueryExecutorProvider for Transaction {{"
        )?;
        writeln!(
            f,
            "    fn executor(&self) -> std::sync::Arc<dyn ::saola_core::query_core::QueryExecutor + Send + Sync> {{ self.0.executor() }}"
        )?;
        writeln!(
            f,
            "    fn query_schema(&self) -> std::sync::Arc<::saola_core::schema::QuerySchema> {{ self.0.query_schema() }}"
        )?;
        writeln!(
            f,
            "    fn tx_id(&self) -> Option<&::saola_core::query_core::TxId> {{ self.0.tx_id() }}"
        )?;
        writeln!(f, "}}")?;
        writeln!(f, "")?;

        writeln!(f, "pub async fn client() -> ::saola_core::Result<SaolaClient> {{")?;
        writeln!(f, "    Ok(SaolaClient(::saola_core::SaolaClient::new(")?;
        writeln!(f, "        r#\"{}\"#,", self.schema_content)?;
        writeln!(f, "        {}", url_tokens)?;
        writeln!(f, "    ).await?))")?;
        writeln!(f, "}}")?;

        // Re-export model queries for the SaolaClient and Transaction
        writeln!(f, "#[allow(dead_code)]")?;
        writeln!(f, "impl SaolaClient {{")?;
        for walker in self.schema.db.walk_models() {
            let model_name_str = walker.name();
            let name_snake = heck::ToSnakeCase::to_snake_case(model_name_str);
            writeln!(
                f,
                "    pub fn {}(&self) -> {}_dir::builders::{}Query {{ {}_dir::builders::{}Query {{ provider: std::sync::Arc::new(self.clone()) }} }}",
                name_snake, name_snake, model_name_str, name_snake, model_name_str
            )?;
        }

        writeln!(
            f,
            "    pub async fn transaction<F, Fut, T>(&self, callback: F) -> ::saola_core::Result<T>"
        )?;
        writeln!(f, "    where")?;
        writeln!(f, "        F: FnOnce(Transaction) -> Fut,")?;
        writeln!(f, "        Fut: std::future::Future<Output = ::saola_core::Result<T>>,")?;
        writeln!(f, "    {{")?;
        writeln!(f, "        self.0.transaction(|tx| callback(Transaction(tx))).await")?;
        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;

        writeln!(f, "#[allow(dead_code)]")?;
        writeln!(f, "impl Transaction {{")?;
        for walker in self.schema.db.walk_models() {
            let model_name_str = walker.name();
            let name_snake = heck::ToSnakeCase::to_snake_case(model_name_str);
            writeln!(
                f,
                "    pub fn {}(&self) -> {}_dir::builders::{}Query {{ {}_dir::builders::{}Query {{ provider: std::sync::Arc::new(self.clone()) }} }}",
                name_snake, name_snake, model_name_str, name_snake, model_name_str
            )?;
        }
        writeln!(f, "}}")?;

        Ok(())
    }

    fn generate_enums_rs(&self, output_dir: &Path) -> anyhow::Result<()> {
        let mut f = fs::File::create(output_dir.join("enums.rs"))?;
        writeln!(f, "#![allow(warnings)]")?;
        writeln!(f, "use ::saola_core::query_structure::PrismaValue;")?;

        for walker in self.schema.db.walk_enums() {
            let code = crate::model_gen::generate_enum(&self.schema.db, walker);
            write!(f, "{}", code)?;
        }
        Ok(())
    }

    fn generate_model_mod_rs(&self, model_dir: &Path, model_name: &str) -> anyhow::Result<()> {
        let mut f = fs::File::create(model_dir.join("mod.rs"))?;
        let name_snake = heck::ToSnakeCase::to_snake_case(model_name);
        writeln!(f, "#![allow(warnings)]")?;
        writeln!(f, "pub mod model;")?;
        writeln!(f, "pub mod builders;")?;
        writeln!(f, "")?;
        writeln!(f, "pub mod mod_exports {{")?;
        writeln!(f, "    pub use super::model::*;")?;
        writeln!(f, "    pub use super::builders::*;")?;
        writeln!(f, "    pub use super::model::_{} as {};", name_snake, name_snake)?;
        writeln!(f, "}}")?;
        Ok(())
    }

    fn generate_model_struct_rs(
        &self,
        model_dir: &Path,
        walker: psl::parser_database::walkers::ModelWalker<'_>,
        metadata: &std::collections::HashMap<String, crate::model_analysis::ModelMetadata>,
    ) -> anyhow::Result<()> {
        let mut f = fs::File::create(model_dir.join("model.rs"))?;
        writeln!(f, "use super::super::enums;")?;
        writeln!(f, "use ::saola_core::serde;")?;

        let code = crate::model_gen::generate_model_struct(&self.schema.db, walker, metadata);
        write!(f, "{}", code)?;
        Ok(())
    }

    fn generate_model_builders_rs(
        &self,
        model_dir: &Path,
        walker: psl::parser_database::walkers::ModelWalker<'_>,
        metadata: &std::collections::HashMap<String, crate::model_analysis::ModelMetadata>,
    ) -> anyhow::Result<()> {
        let mut f = fs::File::create(model_dir.join("builders.rs"))?;

        // Import all re-exports from saola/mod.rs
        writeln!(f, "use super::super::*;")?;
        writeln!(f, "use ::saola_core::prelude::*;")?;

        let model_name_str = walker.name();
        let model_name = quote::format_ident!("{}", model_name_str);
        let model_name_snake = quote::format_ident!("{}", heck::ToSnakeCase::to_snake_case(model_name_str));
        let model_metadata = metadata.get(model_name_str).unwrap();

        write!(
            f,
            "{}",
            crate::builder_gen::generate_model_marker(&model_name, model_metadata)
        )?;
        write!(
            f,
            "{}",
            crate::builder_gen::generate_where_builder(&model_name, model_metadata)
        )?;
        write!(
            f,
            "{}",
            crate::builder_gen::generate_unique_where_builder(&model_name, model_metadata)
        )?;
        write!(
            f,
            "{}",
            crate::builder_gen::generate_order_by_builder(&model_name, model_metadata)
        )?;
        write!(
            f,
            "{}",
            crate::builder_gen::generate_select_builder(&model_name, model_metadata)
        )?;
        write!(
            f,
            "{}",
            crate::builder_gen::generate_include_builder(&model_name, model_metadata)
        )?;
        write!(
            f,
            "{}",
            crate::builder_gen::generate_data_builder(&model_name, model_metadata, Some(metadata))
        )?;
        write!(
            f,
            "{}",
            crate::builder_gen::generate_aggregate_select_builders(&model_name, model_metadata)
        )?;
        write!(
            f,
            "{}",
            crate::builder_gen::generate_group_by_builder(&model_name, model_metadata)
        )?;
        write!(
            f,
            "{}",
            crate::query_gen::generate_query_factory(&model_name, &model_name_snake, model_metadata)
        )?;

        Ok(())
    }
}
