use minijinja::{Environment, Value};
use minijinja::value::MutableList;

#[test]
fn test_what_type_is_created() {
    let env = Environment::new();
    
    let tmpl = env.template_from_str(r#"
        {%- set items = [] -%}
        {{ items }}
    "#).unwrap();
    
    let result = tmpl.render(minijinja::context!{}).unwrap();
    println!("Result: {}", result);
    
    // Now let's manually create a MutableList and see if it can call append
    let mlist = Value::from_object(MutableList::new());
    println!("MutableList type: {:?}", mlist.kind());
    println!("Is object: {:?}", mlist.as_object().is_some());
}

#[test]
fn test_vm_buildlist_directly() {
    // Let's test what BuildList with count=0 actually creates
    let env = Environment::new();
    
    // This should trigger BuildList with count 0
    let tmpl = env.template_from_str("{{ [] }}").unwrap();
    let result = tmpl.render(minijinja::context!{}).unwrap();
    println!("Empty list result: {}", result);
}

