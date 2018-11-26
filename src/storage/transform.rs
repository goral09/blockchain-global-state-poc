use super::value::Value;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Transform {
    Identity,
    Write(Value),
    Add(i32),
}

use self::Transform::*;
use super::utils::{Semigroup, Monoid};

impl Transform {
    pub fn apply(&self, v: Value) -> Result<Value, super::Error> {
        match self {
            Identity => Ok(v),
            Write(w) => Ok(w.clone()),
            Add(i) => match v {
                Value::Int32(j) => Ok(Value::Int32(i + j)),
                other => {
                    let expected = "Int32".to_string();
                    Err(super::Error::TypeMismatch {
                        expected,
                        found: other.type_string(),
                    })
                }
            },
        }
    }
}

impl Semigroup for Transform {
    fn zero() -> Self {
        Identity
    }
}

impl Monoid for Transform {
    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (a, Identity) => a,
            (Identity, b) => b,
            (_, b @ Write(_)) => b,
            (Transform::Add(i), Transform::Add(j)) => Transform::Add(i + j),
            (Write(v), Add(j)) => match v {
                Value::Int32(i) => Write(Value::Int32(i + j)),
                other => Write(other),
            },
        }
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
