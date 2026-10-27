use std::ops::{Add, Mul};

/// A scalar that remembers how it was computed, so gradients can flow back through it.
// The fields are yours to design: how a node holds its children and a mutable grad is the
// core Rust question of this lecture (Rc<RefCell<_>>, an arena with indices, ...).
pub struct Value;

impl Value {
    pub fn new(data: f64) -> Self {
        todo!()
    }

    pub fn data(&self) -> f64 {
        todo!()
    }

    pub fn grad(&self) -> f64 {
        todo!()
    }

    /// Fills in `grad` for every node this value depends on.
    pub fn backward(&self) {
        todo!()
    }
}

impl Add for &Value {
    type Output = Value;

    fn add(self, rhs: &Value) -> Value {
        todo!()
    }
}

impl Mul for &Value {
    type Output = Value;

    fn mul(self, rhs: &Value) -> Value {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The first worked example in the lecture: L = (a*b + c) * f.
    #[test]
    fn lecture_expression() {
        let a = Value::new(2.0);
        let b = Value::new(-3.0);
        let c = Value::new(10.0);
        let f = Value::new(-2.0);
        let e = &a * &b;
        let d = &e + &c;
        let l = &d * &f;

        assert_eq!(l.data(), -8.0);

        l.backward();
        assert_eq!(l.grad(), 1.0);
        assert_eq!(f.grad(), 4.0);
        assert_eq!(d.grad(), -2.0);
        assert_eq!(e.grad(), -2.0);
        assert_eq!(c.grad(), -2.0);
        assert_eq!(a.grad(), 6.0);
        assert_eq!(b.grad(), -4.0);
    }

    // The bug the lecture hits later: a node used twice must accumulate, not overwrite.
    #[test]
    fn reused_node_accumulates_grad() {
        let a = Value::new(3.0);
        let b = &a + &a;

        b.backward();
        assert_eq!(a.grad(), 2.0);
    }
}
