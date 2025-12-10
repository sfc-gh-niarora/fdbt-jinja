use std::collections::BTreeSet;
use std::fmt;
use std::sync::Arc;

use crate::error::{Error, ErrorKind};
use crate::output::Output;
use crate::utils::AutoEscape;
use crate::value::{Enumerator, Kwargs, Object, Value};
use crate::vm::state::State;
use crate::vm::Vm;

/// A macro object that can be invoked from templates
///
/// Macros are reusable template fragments that can be called like functions.
/// They support parameters, default values, and can access the template environment.
pub struct Macro {
    pub name: Value,
    pub arg_spec: Vec<Value>,
    // because values need to be 'static, we can't hold a reference to the
    // instructions that declared the macro.  Instead of that we place the
    // reference to the macro instruction (and the jump offset) in the
    // state under `state.macros`.
    pub(crate) macro_ref_id: usize,
    pub(crate) state_id: isize,
    pub(crate) closure: Value,
    pub(crate) caller_reference: bool,
}

impl fmt::Debug for Macro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<macro {}>", self.name)
    }
}

impl Object for Macro {
    fn enumerate(self: &Arc<Self>) -> Enumerator {
        Enumerator::Str(&["name", "arguments", "caller"])
    }

    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        Some(match some!(key.as_str()) {
            "name" => self.name.clone(),
            "arguments" => Value::from_iter(self.arg_spec.iter().cloned()),
            "caller" => Value::from(self.caller_reference),
            _ => return None,
        })
    }

    fn call(self: &Arc<Self>, state: &State<'_, '_>, args: &[Value]) -> Result<Value, Error> {
        // we can only call macros that point to loaded template state.
        if state.id != self.state_id {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "cannot call this macro. template state went away.",
            ));
        }

        let (args, kwargs) = match args.last() {
            Some(last) => match Kwargs::extract(last) {
                Some(kwargs) => (&args[..args.len() - 1], Some(kwargs)),
                None => (args, None),
            },
            _ => (args, None),
        };

        if args.len() > self.arg_spec.len() {
            return Err(Error::from(ErrorKind::TooManyArguments));
        }

        let mut kwargs_used = BTreeSet::new();
        let mut arg_values = Vec::with_capacity(self.arg_spec.len());
        for (idx, name) in self.arg_spec.iter().enumerate() {
            let name = match name.as_str() {
                Some(name) => name,
                None => {
                    arg_values.push(Value::UNDEFINED);
                    continue;
                }
            };
            let kwarg: Option<&Value> = match kwargs {
                Some(ref kwargs) => kwargs.get(name).ok(),
                _ => None,
            };
            arg_values.push(match (args.get(idx), kwarg) {
                (Some(_), Some(_)) => {
                    return Err(Error::new(
                        ErrorKind::TooManyArguments,
                        format!("duplicate argument `{name}`"),
                    ))
                }
                (Some(arg), None) => arg.clone(),
                (None, Some(kwarg)) => {
                    kwargs_used.insert(name as &str);
                    kwarg.clone()
                }
                (None, None) => Value::UNDEFINED,
            });
        }

        let caller = if self.caller_reference {
            kwargs_used.insert("caller");
            Some(
                kwargs
                    .as_ref()
                    .and_then(|x| x.get("caller").ok())
                    .unwrap_or(Value::UNDEFINED),
            )
        } else {
            None
        };

        if let Some(kwargs) = kwargs {
            for key in kwargs.values.keys().filter_map(|x| x.as_str()) {
                if !kwargs_used.contains(key) {
                    return Err(Error::new(
                        ErrorKind::TooManyArguments,
                        format!("unknown keyword argument `{key}`"),
                    ));
                }
            }
        }

        let vm = Vm::new(state.env());
        let mut rv = String::new();

        // Eval the macro - it may return a typed value via {% return value %}
        // or output a string via normal template rendering
        let typed_return = ok!(vm.eval_macro(
            state,
            self.macro_ref_id,
            &mut Output::new(&mut rv),
            self.closure.clone(),
            caller,
            arg_values
        ));

        // If macro used {% return value %}, return that typed value directly
        // This preserves types (integers, lists, etc.) instead of converting to string
        if let Some(return_value) = typed_return {
            return Ok(return_value);
        }

        // Otherwise return the string output (backward compatible)
        Ok(if !matches!(state.auto_escape(), AutoEscape::None) {
            Value::from_safe_string(rv)
        } else {
            Value::from(rv)
        })
    }

    fn render(self: &Arc<Self>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<macro {}>", self.name)
    }
}
