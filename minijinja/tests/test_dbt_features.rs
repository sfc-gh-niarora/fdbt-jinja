/// Tests for DBT-specific features added to minijinja
use minijinja::{context, Environment, Value};
use minijinja::value::MutableList;

#[test]
fn test_do_statement_exists() {
    let env = Environment::new();
    let tmpl = env.template_from_str(r#"
        {%- set x = 5 -%}
        {%- do print(x) -%}
        done
    "#);
    
    // Should parse without error
    assert!(tmpl.is_ok());
}

#[test]
fn test_mutable_list_append() {
    let mut env = Environment::new();
    
    // Create a MutableList and add it to context
    let items = Value::from_object(MutableList::new());
    
    let tmpl = env.template_from_str(r#"
        {%- do items.append(1) -%}
        {%- do items.append(2) -%}
        {%- do items.append(3) -%}
        {{ items }}
    "#).unwrap();
    
    let result = tmpl.render(context! { items }).unwrap();
    assert_eq!(result.trim(), "[1, 2, 3]");
}

#[test]
fn test_mutable_list_extend() {
    let mut env = Environment::new();
    
    let items = Value::from_object(MutableList::new());
    
    let tmpl = env.template_from_str(r#"
        {%- do items.append(1) -%}
        {%- do items.extend([2, 3, 4]) -%}
        {{ items }}
    "#).unwrap();
    
    let result = tmpl.render(context! { items }).unwrap();
    assert_eq!(result.trim(), "[1, 2, 3, 4]");
}

#[test]
fn test_mutable_list_insert() {
    let mut env = Environment::new();
    
    let items = Value::from_object(MutableList::new());
    
    let tmpl = env.template_from_str(r#"
        {%- do items.append(1) -%}
        {%- do items.append(3) -%}
        {%- do items.insert(1, 2) -%}
        {{ items }}
    "#).unwrap();
    
    let result = tmpl.render(context! { items }).unwrap();
    assert_eq!(result.trim(), "[1, 2, 3]");
}

#[test]
fn test_mutable_list_pop() {
    let mut env = Environment::new();
    
    let items = Value::from_object(MutableList::new());
    
    let tmpl = env.template_from_str(r#"
        {%- do items.append(1) -%}
        {%- do items.append(2) -%}
        {%- do items.append(3) -%}
        {%- set popped = items.pop() -%}
        {{ items }},{{ popped }}
    "#).unwrap();
    
    let result = tmpl.render(context! { items }).unwrap();
    assert_eq!(result.trim(), "[1, 2],3");
}

#[test]
fn test_mutable_list_clear() {
    let mut env = Environment::new();
    
    let items = Value::from_object(MutableList::new());
    
    let tmpl = env.template_from_str(r#"
        {%- do items.append(1) -%}
        {%- do items.append(2) -%}
        {%- do items.clear() -%}
        {{ items }}
    "#).unwrap();
    
    let result = tmpl.render(context! { items }).unwrap();
    assert_eq!(result.trim(), "[]");
}

#[test]
fn test_macro_with_kwargs() {
    let env = Environment::new();
    let tmpl = env.template_from_str(r#"
        {%- macro greet(name, greeting) -%}
        {{ greeting }} {{ name }}!
        {%- endmacro -%}
        {{ greet(name="World", greeting="Hello") }}
    "#).unwrap();
    
    let result = tmpl.render(context!{}).unwrap();
    assert_eq!(result.trim(), "Hello World!");
}

#[test]
fn test_macro_kwargs_with_defaults() {
    let env = Environment::new();
    let tmpl = env.template_from_str(r#"
        {%- macro format_date(date, fmt) -%}
        {{ date }}|{{ fmt }}
        {%- endmacro -%}
        {{ format_date(date="2024-01-01", fmt="YYYY-MM-DD") }}
        {{ format_date(fmt="MM/DD/YYYY", date="2024-12-25") }}
    "#).unwrap();
    
    let result = tmpl.render(context!{}).unwrap();
    let lines: Vec<&str> = result.trim().lines().map(|l| l.trim()).collect();
    assert_eq!(lines[0], "2024-01-01|YYYY-MM-DD");
    assert_eq!(lines[1], "2024-12-25|MM/DD/YYYY");
}

#[test]
fn test_nested_macro_calls() {
    let env = Environment::new();
    
    let tmpl = env.template_from_str(r#"
        {%- macro inner(x) -%}{{ x * 2 }}{%- endmacro -%}
        {%- macro outer(y) -%}{{ inner(y + 1) }}{%- endmacro -%}
        {{ outer(5) }}
    "#).unwrap();
    
    let result = tmpl.render(context!{}).unwrap();
    assert_eq!(result.trim(), "12");
}

#[test]
fn test_auto_mutable_list() {
    let env = Environment::new();
    
    // Test that empty [] creates a MutableList automatically
    let tmpl = env.template_from_str(r#"
        {%- set items = [] -%}
        {%- do items.append(1) -%}
        {%- do items.append(2) -%}
        {%- do items.append(3) -%}
        {{ items }}
    "#).unwrap();
    
    let result = tmpl.render(context!{}).unwrap();
    assert_eq!(result.trim(), "[1, 2, 3]");
}

#[test]
fn test_auto_mutable_list_with_extend() {
    let env = Environment::new();
    
    let tmpl = env.template_from_str(r#"
        {%- set fields = [] -%}
        {%- do fields.append("id") -%}
        {%- do fields.append("name") -%}
        {%- do fields.extend(["email", "created_at"]) -%}
        {{ fields }}
    "#).unwrap();
    
    let result = tmpl.render(context!{}).unwrap();
    assert_eq!(result.trim(), r#"["id", "name", "email", "created_at"]"#);
}

#[test]
fn test_non_empty_list_still_works() {
    let env = Environment::new();
    
    // Non-empty lists should still work normally
    let tmpl = env.template_from_str(r#"
        {%- set items = [1, 2, 3] -%}
        {{ items }}
    "#).unwrap();
    
    let result = tmpl.render(context!{}).unwrap();
    assert_eq!(result.trim(), "[1, 2, 3]");
}

