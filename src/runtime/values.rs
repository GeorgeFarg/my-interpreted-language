use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone)]
pub enum ValueType {
    Null,
    Number,
    Boolean,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum RuntimeValue {
    NullVal(NullVal),
    NumberVal(NumberVal),
    BooleanVal(BoolVal),
}

#[derive(Debug, Clone)]
pub struct NullVal {
    #[allow(unused)]
    val_type: ValueType,
}

impl NullVal {
    pub fn new() -> Self {
        return Self {
            val_type: ValueType::Null,
        };
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct BoolVal {
    val_type: ValueType,
    value: bool,
}

impl BoolVal {
    pub fn new(value: bool) -> Self {
        return Self {
            val_type: ValueType::Boolean,
            value,
        };
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
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
