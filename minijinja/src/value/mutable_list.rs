/// Mutable list implementation for DBT compatibility
///
/// This module provides a mutable list type that supports in-place operations
/// like append(), extend(), insert(), pop(), and clear(). This is needed for
/// DBT templates that use {% do list.append(item) %} patterns.
use std::sync::{Arc, Mutex};

use crate::value::{DynObject, Enumerator, Object, ObjectRepr, Value};
use crate::State;

/// A mutable list that can be modified in place via template operations.
///
/// # Example
/// ```jinja
/// {%- set items = [] -%}
/// {%- do items.append(1) -%}
/// {%- do items.append(2) -%}
/// {{ items }}  {# outputs: [1, 2] #}
/// ```
#[derive(Debug, Clone)]
pub struct MutableList {
    inner: Arc<Mutex<Vec<Value>>>,
}

impl MutableList {
    /// Creates a new empty mutable list.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Creates a mutable list from an existing vec.
    pub fn from_vec(vec: Vec<Value>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(vec)),
        }
    }

    /// Appends an item to the end of the list.
    pub fn append(&self, item: Value) {
        if let Ok(mut list) = self.inner.lock() {
            list.push(item);
        }
    }

    /// Extends the list by appending all items from an iterable.
    pub fn extend(&self, items: Value) {
        if let Ok(mut list) = self.inner.lock() {
            // Try to iterate over the value
            if let Ok(iter) = items.try_iter() {
                for item in iter {
                    list.push(item);
                }
            } else {
                // If not iterable, just append the single item
                list.push(items);
            }
        }
    }

    /// Inserts an item at the specified index.
    pub fn insert(&self, index: usize, item: Value) {
        if let Ok(mut list) = self.inner.lock() {
            if index <= list.len() {
                list.insert(index, item);
            }
        }
    }

    /// Removes and returns the item at the specified index.
    /// If no index is provided, removes the last item.
    pub fn pop(&self, index: Option<usize>) -> Value {
        if let Ok(mut list) = self.inner.lock() {
            match index {
                Some(idx) if idx < list.len() => list.remove(idx),
                None if !list.is_empty() => list.pop().unwrap(),
                _ => Value::UNDEFINED,
            }
        } else {
            Value::UNDEFINED
        }
    }

    /// Removes all items from the list.
    pub fn clear(&self) {
        if let Ok(mut list) = self.inner.lock() {
            list.clear();
        }
    }

    /// Returns the number of items in the list.
    pub fn len(&self) -> usize {
        self.inner.lock().map(|list| list.len()).unwrap_or(0)
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets a copy of the item at the specified index.
    pub fn get(&self, index: usize) -> Option<Value> {
        self.inner
            .lock()
            .ok()
            .and_then(|list| list.get(index).cloned())
    }

    /// Returns a snapshot of the list as a Vec.
    pub fn to_vec(&self) -> Vec<Value> {
        self.inner
            .lock()
            .map(|list| list.clone())
            .unwrap_or_default()
    }
}

impl Object for MutableList {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Seq
    }

    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        let index = key.as_usize()?;
        self.get(index)
    }

    fn enumerate(self: &Arc<Self>) -> Enumerator {
        let len = self.len();
        Enumerator::Seq(len)
    }

    fn call_method(
        self: &Arc<Self>,
        _state: &State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, crate::Error> {
        match name {
            "append" => {
                if let Some(item) = args.first() {
                    self.append(item.clone());
                    Ok(Value::UNDEFINED)
                } else {
                    Err(crate::Error::new(
                        crate::ErrorKind::MissingArgument,
                        "append() requires 1 argument",
                    ))
                }
            }
            "extend" => {
                if let Some(items) = args.first() {
                    self.extend(items.clone());
                    Ok(Value::UNDEFINED)
                } else {
                    Err(crate::Error::new(
                        crate::ErrorKind::MissingArgument,
                        "extend() requires 1 argument",
                    ))
                }
            }
            "insert" => {
                if args.len() >= 2 {
                    if let Some(index) = args[0].as_usize() {
                        self.insert(index, args[1].clone());
                        Ok(Value::UNDEFINED)
                    } else {
                        Err(crate::Error::new(
                            crate::ErrorKind::InvalidOperation,
                            "insert() first argument must be an integer",
                        ))
                    }
                } else {
                    Err(crate::Error::new(
                        crate::ErrorKind::MissingArgument,
                        "insert() requires 2 arguments",
                    ))
                }
            }
            "pop" => {
                let index = args.first().and_then(|v| v.as_usize());
                Ok(self.pop(index))
            }
            "clear" => {
                self.clear();
                Ok(Value::UNDEFINED)
            }
            _ => Err(crate::Error::from(crate::ErrorKind::UnknownMethod)),
        }
    }
}

impl Default for MutableList {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to check if a Value is a MutableList
pub fn is_mutable_list(value: &Value) -> bool {
    value.downcast_object_ref::<MutableList>().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutable_list_append() {
        let list = MutableList::new();
        list.append(Value::from(1));
        list.append(Value::from(2));
        list.append(Value::from(3));

        assert_eq!(list.len(), 3);
        assert_eq!(list.get(0), Some(Value::from(1)));
        assert_eq!(list.get(1), Some(Value::from(2)));
        assert_eq!(list.get(2), Some(Value::from(3)));
    }

    #[test]
    fn test_mutable_list_extend() {
        let list = MutableList::new();
        list.append(Value::from(1));

        let items = Value::from(vec![2, 3, 4]);
        list.extend(items);

        assert_eq!(list.len(), 4);
        assert_eq!(list.to_vec(), vec![
            Value::from(1),
            Value::from(2),
            Value::from(3),
            Value::from(4)
        ]);
    }

    #[test]
    fn test_mutable_list_insert() {
        let list = MutableList::new();
        list.append(Value::from(1));
        list.append(Value::from(3));
        list.insert(1, Value::from(2));

        assert_eq!(list.len(), 3);
        assert_eq!(list.to_vec(), vec![
            Value::from(1),
            Value::from(2),
            Value::from(3)
        ]);
    }

    #[test]
    fn test_mutable_list_pop() {
        let list = MutableList::new();
        list.append(Value::from(1));
        list.append(Value::from(2));
        list.append(Value::from(3));

        let popped = list.pop(None);
        assert_eq!(popped, Value::from(3));
        assert_eq!(list.len(), 2);

        let popped = list.pop(Some(0));
        assert_eq!(popped, Value::from(1));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_mutable_list_clear() {
        let list = MutableList::new();
        list.append(Value::from(1));
        list.append(Value::from(2));
        list.clear();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }
}

