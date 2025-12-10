/// Test that MutableList works correctly inside macros
///
/// This is critical for DBT's generate_surrogate_key macro which uses
/// {% do fields.append(...) %} inside a macro body.

use minijinja::Environment;

#[test]
fn test_mutable_list_append_in_macro() {
    let template_str = r#"
{% macro build_list(items) %}
{% set fields = [] %}
{% for item in items %}
    {% do fields.append("col_" ~ item) %}
{% endfor %}
{{ fields | join(' || ') }}
{% endmacro %}

Result: {{ build_list(['id', 'name']) }}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    
    println!("Result: {:?}", result);
    
    // Should output: col_id || col_name
    assert!(result.contains("col_id"), "Expected 'col_id' in output, got: {}", result);
    assert!(result.contains("col_name"), "Expected 'col_name' in output, got: {}", result);
    assert!(result.contains("||"), "Expected '||' in output, got: {}", result);
}

#[test]
fn test_mutable_list_append_outside_macro() {
    let template_str = r#"
{% set fields = [] %}
{% do fields.append("test1") %}
{% do fields.append("test2") %}
{{ fields | join(' || ') }}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    
    println!("Result: {:?}", result);
    
    // Should work outside macro
    assert!(result.contains("test1"));
    assert!(result.contains("test2"));
}

