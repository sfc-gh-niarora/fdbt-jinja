/// Test the exact scenario from jaffle-shop's test_kwargs model
use minijinja::{context, Environment, Value};
use minijinja::value::MutableList;

#[test]
fn test_get_base_dates_simplified() {
    let env = Environment::new();
    
    // Simplified version of get_base_dates macro
    let tmpl = env.template_from_str(r#"
        {%- macro get_base_dates(start_date, end_date, n_dateparts, datepart) -%}
        {%- if n_dateparts and datepart -%}
        with date_spine as (select {{ datepart }} from dual)
        select * from date_spine
        {%- endif -%}
        {%- endmacro -%}
        
        {{ get_base_dates(n_dateparts=100, datepart="day") }}
    "#).unwrap();
    
    let result = tmpl.render(context!{}).unwrap();
    println!("Result: {}", result);
    assert!(result.contains("select"));
}

#[test]
fn test_nested_macro_with_kwargs() {
    let env = Environment::new();
    
    // Macro that calls another macro
    let tmpl = env.template_from_str(r#"
        {%- macro inner(value, multiplier) -%}
        {{ value * multiplier }}
        {%- endmacro -%}
        
        {%- macro outer(x, y) -%}
        {{ inner(value=x, multiplier=y) }}
        {%- endmacro -%}
        
        {{ outer(x=5, y=10) }}
    "#).unwrap();
    
    let result = tmpl.render(context!{}).unwrap();
    assert_eq!(result.trim(), "50");
}

#[test]
fn test_macro_with_list_mutation() {
    let env = Environment::new();
    
    // Macro that uses do and list.append
    let tmpl = env.template_from_str(r#"
        {%- macro build_list(items) -%}
        {%- set result = [] -%}
        {%- for item in items -%}
        {%- do result.append(item) -%}
        {%- endfor -%}
        {{ result }}
        {%- endmacro -%}
        
        {{ build_list(['a', 'b', 'c']) }}
    "#).unwrap();
    
    let result = tmpl.render(context!{}).unwrap();
    assert_eq!(result.trim(), r#"["a", "b", "c"]"#);
}

