/// Test that macros can return typed values (not just strings)
/// 
/// This is essential for DBT where macros like get_powers_of_two() need
/// to return integers that can be used in range() calls.

use minijinja::Environment;

#[test]
fn test_macro_returns_integer() {
    let template_str = r#"
{%- macro get_number() -%}
{% return 42 %}
{%- endmacro -%}

{% set n = get_number() %}
Type: {{ n is number }}
Value: {{ n }}
Math: {{ n * 2 }}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    println!("Result: {:?}", result);
    
    assert!(result.contains("Type: true"));
    assert!(result.contains("Value: 42"));
    assert!(result.contains("Math: 84"));
}

#[test]
fn test_macro_returns_integer_for_range() {
    let template_str = r#"
{%- macro get_count() -%}
{% return 5 %}
{%- endmacro -%}

{% set n = get_count() %}
{% for i in range(n) %}{{ i }}{% if not loop.last %},{% endif %}{% endfor %}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    println!("Result: {:?}", result);
    
    assert_eq!(result.trim(), "0,1,2,3,4");
}

#[test]
fn test_nested_macro_with_typed_return() {
    let template_str = r#"
{%- macro inner() -%}
{% return 10 %}
{%- endmacro -%}

{%- macro outer() -%}
{% set x = inner() %}
{% return x * 2 %}
{%- endmacro -%}

Result: {{ outer() }}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    println!("Result: {:?}", result);
    
    assert!(result.contains("Result: 20"));
}

#[test]
fn test_macro_returns_list() {
    let template_str = r#"
{%- macro get_list() -%}
{% return [1, 2, 3] %}
{%- endmacro -%}

{% set items = get_list() %}
{% for item in items %}{{ item }}{% endfor %}
"#;

    let env = Environment::new();
    let template = env.template_from_str(template_str).unwrap();
    let result = template.render(minijinja::context!{}).unwrap();
    println!("Result: {:?}", result);
    
    assert_eq!(result.trim(), "123");
}

