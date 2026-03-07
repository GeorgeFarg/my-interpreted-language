use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone)]
pub enum ValueType {
    Null,
    Number,
}

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    NullVal(NullVal),
    NumberVal(NumberVal),
}

#[derive(Debug, Clone)]
pub struct NullVal {
    val_type: ValueType,
}

impl NullVal {
    pub fn new() -> Self {
        return NullVal {
            val_type: ValueType::Null,
        };
    }
}

#[derive(Debug, Clone)]
pub struct NumberVal {
    val_type: ValueType,
    value: f64,
}

impl NumberVal {
    pub fn new(val_type: ValueType, value: f64) -> Self {
        return NumberVal { val_type, value };
    }
}

impl Add for NumberVal {
    type Output = NumberVal;

    fn add(self, other: NumberVal) -> Self::Output {
        NumberVal {
            val_type: ValueType::Number,
            value: self.value + other.value,
        }
    }
}

impl Sub for NumberVal {
    type Output = NumberVal;

    fn sub(self, other: NumberVal) -> Self::Output {
        NumberVal {
            val_type: ValueType::Number,
            value: self.value - other.value,
        }
    }
}

impl Mul for NumberVal {
    type Output = NumberVal;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            val_type: ValueType::Number,
            value: self.value * other.value,
        }
    }
}

impl Div for NumberVal {
    type Output = NumberVal;

    fn div(self, other: Self) -> Self::Output {
        Self {
            val_type: ValueType::Number,
            value: self.value / other.value,
        }
    }
}

impl Rem for NumberVal {
    type Output = NumberVal;

    fn rem(self, other: Self) -> Self::Output {
        Self {
            val_type: ValueType::Number,
            value: self.value % other.value,
        }
    }
}
