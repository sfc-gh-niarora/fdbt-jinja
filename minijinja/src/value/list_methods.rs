//! List mutation methods for DBT compatibility
//!
//! This module adds mutable list methods (.append(), .extend(), etc.)
//! that are needed for DBT template compilation.

use crate::error::{Error, ErrorKind};
use crate::value::Value;
use crate::vm::State;
use std::sync::Arc;
use std::sync::Mutex;

/// Handles list mutation method calls
pub(crate) fn handle_list_method(
    value: &Value,
    state: &State,
    method: &str,
    args: &[Value],
) -> Result<Option<Value>, Error> {
    // Only handle sequences
    if !matches!(value.kind(), crate::value::ValueKind::Seq) {
        return Ok(None);
    }

    match method {
        "append" => {
            // list.append(item) - add item to end of list
            if args.len() != 1 {
                return Err(Error::new(
                    ErrorKind::TooManyArguments,
                    format!("append() takes exactly 1 argument ({} given)", args.len()),
                ));
            }

            // Convert to vector, append, and replace
            let mut items: Vec<Value> = value.try_iter()?.collect();
            items.push(args[0].clone());
            
            // Create new value from extended list
            let new_value = Value::from(items);
            
            // In Jinja2, append() returns None but mutates in place
            // Since Rust values are immutable, we need a different approach
            // We'll return the new list and let the caller handle the assignment
            Ok(Some(new_value))
        }
        
        "extend" => {
            // list.extend(iterable) - add all items from iterable
            if args.len() != 1 {
                return Err(Error::new(
                    ErrorKind::TooManyArguments,
                    format!("extend() takes exactly 1 argument ({} given)", args.len()),
                ));
            }

            let mut items: Vec<Value> = value.try_iter()?.collect();
            
            // Iterate over the argument and append each item
            for item in args[0].try_iter()? {
                items.push(item);
            }
            
            let new_value = Value::from(items);
            Ok(Some(new_value))
        }
        
        "insert" => {
            // list.insert(index, item) - insert item at index
            if args.len() != 2 {
                return Err(Error::new(
                    ErrorKind::TooManyArguments,
                    format!("insert() takes exactly 2 arguments ({} given)", args.len()),
                ));
            }

            let index = args[0].as_usize().ok_or_else(|| {
                Error::new(ErrorKind::InvalidOperation, "insert() index must be an integer")
            })?;

            let mut items: Vec<Value> = value.try_iter()?.collect();
            
            // Clamp index to valid range
            let actual_index = std::cmp::min(index, items.len());
            items.insert(actual_index, args[1].clone());
            
            let new_value = Value::from(items);
            Ok(Some(new_value))
        }
        
        "pop" => {
            // list.pop([index]) - remove and return item at index (default: last)
            if args.len() > 1 {
                return Err(Error::new(
                    ErrorKind::TooManyArguments,
                    format!("pop() takes at most 1 argument ({} given)", args.len()),
                ));
            }

            let mut items: Vec<Value> = value.try_iter()?.collect();
            
            if items.is_empty() {
                return Err(Error::new(ErrorKind::InvalidOperation, "pop from empty list"));
            }

            let index = if args.is_empty() {
                items.len() - 1
            } else {
                args[0].as_usize().ok_or_else(|| {
                    Error::new(ErrorKind::InvalidOperation, "pop() index must be an integer")
                })?
            };

            if index >= items.len() {
                return Err(Error::new(ErrorKind::InvalidOperation, "pop index out of range"));
            }

            let popped = items.remove(index);
            
            // For pop, we need to return both the new list and the popped item
            // But since we can only return one value, we'll return the popped item
            // and the caller needs to handle the list update
            Ok(Some(popped))
        }
        
        "clear" => {
            // list.clear() - remove all items
            if !args.is_empty() {
                return Err(Error::new(
                    ErrorKind::TooManyArguments,
                    format!("clear() takes no arguments ({} given)", args.len()),
                ));
            }

            let new_value = Value::from(Vec::<Value>::new());
            Ok(Some(new_value))
        }
        
        _ => Ok(None), // Not a list method we handle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{context, Environment};

    #[test]
    fn test_append() {
        let value = Value::from(vec![1, 2, 3]);
        let result = handle_list_method(&value, &State::new(/* ... */), "append", &[Value::from(4)]);
        // This test won't compile yet as we need proper State construction
        // Will add proper integration tests in test_dbt_features.rs
    }
}

