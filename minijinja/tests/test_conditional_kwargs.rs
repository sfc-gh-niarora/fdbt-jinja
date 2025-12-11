/// Tests for conditional expressions in keyword arguments
/// 
/// This tests the fix for the parser issue where conditional expressions
/// (if...else) were not allowed in keyword argument positions.
/// 
/// Pattern: func(key=value if condition else default)

use minijinja::{context, Environment};

#[test]
fn test_simple_conditional_kwarg() {
    let mut env = Environment::new();
    env.add_function("test_func", |_kwargs: minijinja::value::Kwargs| -> String {
        _kwargs.get::<String>("key").unwrap_or_default()
    });
    
    let tmpl = env.template_from_str(r#"{{ test_func(key="yes" if true else "no") }}"#).unwrap();
    let result = tmpl.render(context! {}).unwrap();
    assert_eq!(result, "yes");
    
    let tmpl = env.template_from_str(r#"{{ test_func(key="yes" if false else "no") }}"#).unwrap();
    let result = tmpl.render(context! {}).unwrap();
    assert_eq!(result, "no");
}

#[test]
fn test_conditional_kwarg_with_function_call() {
    let mut env = Environment::new();
    env.add_function("inner", |x: String| -> String {
        format!("inner_{}", x)
    });
    env.add_function("outer", |_kwargs: minijinja::value::Kwargs| -> String {
        _kwargs.get::<String>("key").unwrap_or_default()
    });
    
    let tmpl = env.template_from_str(r#"{{ outer(key=inner("test") if true else "default") }}"#).unwrap();
    let result = tmpl.render(context! {}).unwrap();
    assert_eq!(result, "inner_test");
}

#[test]
fn test_conditional_kwarg_with_attribute_access() {
    let mut env = Environment::new();
    
    // Add a function that returns an object with attributes
    env.add_function("get_obj", || -> minijinja::Value {
        minijinja::context! {
            name => "object_name",
            value => 42
        }
    });
    
    env.add_function("process", |_kwargs: minijinja::value::Kwargs| -> String {
        _kwargs.get::<String>("key").unwrap_or_default()
    });
    
    // Test the pattern: func(key=obj.attr if cond else default)
    let tmpl = env.template_from_str(r#"{{ process(key=get_obj().name if true else "default") }}"#).unwrap();
    let result = tmpl.render(context! {}).unwrap();
    assert_eq!(result, "object_name");
    
    let tmpl = env.template_from_str(r#"{{ process(key=get_obj().name if false else "fallback") }}"#).unwrap();
    let result = tmpl.render(context! {}).unwrap();
    assert_eq!(result, "fallback");
}

#[test]
fn test_conditional_kwarg_complex_pattern() {
    // This is the exact pattern from the CHALLENGE_TS_12 failure:
    // func(key=inner(a, b).attr if cond(x, y) else fallback(z))
    
    let mut env = Environment::new();
    
    env.add_function("source", |schema: String, table: String| -> minijinja::Value {
        minijinja::context! {
            database => format!("DB_{}", schema),
            schema => schema,
            identifier => table
        }
    });
    
    env.add_function("var", |key: String, default: Option<String>| -> String {
        if key == "has_defined_sources" {
            "false".to_string()
        } else {
            default.unwrap_or_default()
        }
    });
    
    env.add_function("adapter_get_relation", |_kwargs: minijinja::value::Kwargs| -> String {
        let database = _kwargs.get::<String>("database").unwrap_or_default();
        let schema = _kwargs.get::<String>("schema").unwrap_or_default();
        let identifier = _kwargs.get::<String>("identifier").unwrap_or_default();
        format!("{}.{}.{}", database, schema, identifier)
    });
    
    // Test the complex pattern
    let tmpl = env.template_from_str(r#"
        {%- set schema = "raw" -%}
        {%- set table_identifier = "customers" -%}
        {%- set database_variable = "db_var" -%}
        {%- set default_database = "DEFAULT_DB" -%}
        {{- adapter_get_relation(
            database=source(schema, table_identifier).database if var('has_defined_sources', 'false') == 'true' else var(database_variable, default_database),
            schema=source(schema, table_identifier).schema if var('has_defined_sources', 'false') == 'true' else schema,
            identifier=source(schema, table_identifier).identifier if var('has_defined_sources', 'false') == 'true' else table_identifier
        ) -}}
    "#).unwrap();
    
    let result = tmpl.render(context! {}).unwrap();
    assert_eq!(result.trim(), "DEFAULT_DB.raw.customers");
}

#[test]
fn test_conditional_kwarg_in_macro() {
    let env = Environment::new();
    
    let tmpl = env.template_from_str(r#"
        {%- macro test_macro(schema, table, has_sources, fallback) -%}
            {{- some_func(
                key=inner(schema, table).attr if has_sources else fallback
            ) -}}
        {%- endmacro -%}
        
        {{- test_macro("schema1", "table1", true, "default") -}}
    "#).unwrap();
    
    // This should parse without errors (even if it fails at runtime due to undefined functions)
    assert!(tmpl.render(context! {}).is_err()); // Will fail at runtime, but parsing succeeded
}

#[test]
fn test_multiple_conditional_kwargs() {
    let mut env = Environment::new();
    
    env.add_function("func", |_kwargs: minijinja::value::Kwargs| -> String {
        let a = _kwargs.get::<String>("a").unwrap_or_default();
        let b = _kwargs.get::<String>("b").unwrap_or_default();
        let c = _kwargs.get::<String>("c").unwrap_or_default();
        format!("{},{},{}", a, b, c)
    });
    
    let tmpl = env.template_from_str(r#"{{ func(
        a="A" if true else "X",
        b="B" if false else "Y",
        c="C" if true else "Z"
    ) }}"#).unwrap();
    
    let result = tmpl.render(context! {}).unwrap();
    assert_eq!(result, "A,Y,C");
}

#[test]
fn test_nested_conditionals_in_kwargs() {
    let mut env = Environment::new();
    
    env.add_function("func", |_kwargs: minijinja::value::Kwargs| -> String {
        _kwargs.get::<String>("key").unwrap_or_default()
    });
    
    // Nested conditional: val1 if (cond1 if cond2 else cond3) else val2
    let tmpl = env.template_from_str(r#"{{ func(
        key="A" if (true if true else false) else "B"
    ) }}"#).unwrap();
    
    let result = tmpl.render(context! {}).unwrap();
    assert_eq!(result, "A");
}

