/// MacroNamespace - A namespace object that can store and invoke macros
///
/// This enables patterns like:
/// - dbt_utils.generate_surrogate_key(['id', 'name'])
/// - dbt_date.get_base_dates(n_dateparts=100)
///
/// Unlike the basic Namespace, MacroNamespace stores references to actual
/// Macro objects and can invoke them while preserving the environment context.

use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{Error, ErrorKind};
use crate::value::{Enumerator, Object, Value};
use crate::vm::State;

#[cfg(feature = "macros")]
use crate::vm::macro_object::Macro;

/// A namespace that contains and can invoke macros
#[derive(Debug, Clone)]
pub struct MacroNamespace {
    name: String,
    #[cfg(feature = "macros")]
    macros: Arc<HashMap<String, Arc<Macro>>>,
    #[cfg(not(feature = "macros"))]
    _phantom: std::marker::PhantomData<()>,
}

impl MacroNamespace {
    /// Create a new macro namespace with a name
    #[cfg(feature = "macros")]
    pub fn new(name: String) -> Self {
        Self {
            name,
            macros: Arc::new(HashMap::new()),
        }
    }

    /// Create a macro namespace with a registry of macros
    #[cfg(feature = "macros")]
    pub fn with_macros(name: String, macros: HashMap<String, Arc<Macro>>) -> Self {
        Self {
            name,
            macros: Arc::new(macros),
        }
    }

    /// Get the name of this namespace
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if a macro exists in this namespace
    #[cfg(feature = "macros")]
    pub fn has_macro(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    /// Get a macro from this namespace
    #[cfg(feature = "macros")]
    pub fn get_macro(&self, name: &str) -> Option<Arc<Macro>> {
        self.macros.get(name).cloned()
    }
}

impl Object for MacroNamespace {
    fn repr(self: &Arc<Self>) -> crate::value::ObjectRepr {
        crate::value::ObjectRepr::Map
    }

    fn enumerate(self: &Arc<Self>) -> Enumerator {
        #[cfg(feature = "macros")]
        {
            let keys: Vec<Value> = self
                .macros
                .keys()
                .map(|k| Value::from(k.as_str()))
                .collect();
            Enumerator::Values(keys)
        }
        #[cfg(not(feature = "macros"))]
        {
            Enumerator::Empty
        }
    }

    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        #[cfg(feature = "macros")]
        {
            let _key_str = key.as_str()?;
            // Macros are stored as Arc<Macro>, but Value::from_object needs
            // the Macro itself. However, Macro doesn't implement Clone.
            // For now, we don't support direct attribute access to macros.
            // They should be called as methods instead.
            None
        }
        #[cfg(not(feature = "macros"))]
        {
            let _ = key;
            None
        }
    }

    fn call_method(
        self: &Arc<Self>,
        state: &State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        #[cfg(feature = "macros")]
        {
            // Try to find the macro in our registry
            if let Some(macro_obj) = self.macros.get(name) {
                // Call the macro with the current state
                // This preserves the environment context
                return macro_obj.call(state, args);
            }

            // Macro not found
            Err(Error::new(
                ErrorKind::UnknownMethod,
                format!(
                    "namespace '{}' has no method '{}'",
                    self.name, name
                ),
            ))
        }
        #[cfg(not(feature = "macros"))]
        {
            let _ = (state, args);
            Err(Error::new(
                ErrorKind::UnknownMethod,
                format!(
                    "namespace '{}' has no method '{}' (macros not enabled)",
                    self.name, name
                ),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_namespace_creation() {
        let ns = MacroNamespace::new("test".to_string());
        assert_eq!(ns.name(), "test");
    }

    #[cfg(feature = "macros")]
    #[test]
    fn test_macro_namespace_has_macro() {
        let ns = MacroNamespace::new("test".to_string());
        assert!(!ns.has_macro("foo"));
    }
}

