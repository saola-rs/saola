use crate::common::*;
use saola_psl::parser_database::ScalarType;

#[test]
fn should_not_remove_whitespace() {
    let dml = indoc! {r#"
        model User {
          id        Int    @id
          firstName String @default("This is a string with whitespace")
        }
    "#};

    saola_psl::parse_schema_without_extensions(dml)
        .unwrap()
        .assert_has_model("User")
        .assert_has_scalar_field("firstName")
        .assert_scalar_type(ScalarType::String)
        .assert_default_value()
        .assert_string("This is a string with whitespace");
}

#[test]
fn should_not_try_to_interpret_comments_in_strings() {
    let dml = indoc! {r#"
        model User {
          id        Int    @id
          firstName String @default("This is a string with a // Comment")
        }
    "#};

    saola_psl::parse_schema_without_extensions(dml)
        .unwrap()
        .assert_has_model("User")
        .assert_has_scalar_field("firstName")
        .assert_scalar_type(ScalarType::String)
        .assert_default_value()
        .assert_string("This is a string with a // Comment");
}
