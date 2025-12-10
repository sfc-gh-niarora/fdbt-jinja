/// Test that the {% do %} statement works correctly
///
/// The {% do %} statement evaluates an expression without outputting it.
/// This is commonly used for side effects like list.append(), dict.update(), etc.

use minijinja::Environment;

#[test]
fn test_do_statement_with_append() {
    let template_str = r#"
{% set items = [] %}
{% do items.append("apple") %}
{% do items.append("banana") %}
Result: {{ items | join(", ") }}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    
    assert!(result.contains("apple, banana"), "Expected 'apple, banana', got: {}", result);
}

#[test]
fn test_do_statement_in_macro() {
    let template_str = r#"
{% macro build_list(items) %}
{% set result = [] %}
{% for item in items %}
    {% do result.append(item | upper) %}
{% endfor %}
{{ result | join(" | ") }}
{% endmacro %}

Output: {{ build_list(["hello", "world"]) }}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    
    assert!(result.contains("HELLO | WORLD"), "Expected 'HELLO | WORLD', got: {}", result);
}

#[test]
fn test_do_statement_with_method_calls() {
    let template_str = r#"
{% set data = [] %}
{% do data.append(1) %}
{% do data.append(2) %}
{% do data.extend([3, 4]) %}
Count: {{ data | length }}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    
    assert!(result.contains("Count: 4"), "Expected 'Count: 4', got: {}", result);
}

#[test]
fn test_do_statement_with_expression() {
    let template_str = r#"
{% set x = 5 %}
{% do x * 2 %}
X is still: {{ x }}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    
    // do evaluates but doesn't change x or output anything
    assert!(result.contains("X is still: 5"), "Expected 'X is still: 5', got: {}", result);
}

