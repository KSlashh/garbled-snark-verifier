use crate::core::gate::GateCount;
use std::{cell::RefCell, rc::Rc};

pub struct TestCircuit(pub TestWires, pub Vec<TestGate>);

impl TestCircuit {
    pub fn empty() -> Self {
        Self(Vec::new(), Vec::new())
    }

    pub fn new(wires: TestWires, gates: Vec<TestGate>) -> Self {
        Self(wires, gates)
    }

    pub fn extend(&mut self, circuit: Self) -> TestWires {
        self.1.extend(circuit.1);
        circuit.0
    }

    pub fn add(&mut self, gate: TestGate) {
        self.1.push(gate);
    }

    pub fn add_wire(&mut self, wire: TestWirex) {
        self.0.push(wire);
    }

    pub fn add_wires(&mut self, wires: TestWires) {
        self.0.extend(wires);
    }

    pub fn gate_count(&self) -> usize {
        self.1.len()
    }

    pub fn gate_counts(&self) -> GateCount {
        let mut and = 0;
        let mut or = 0;
        let mut xor = 0;
        let mut nand = 0;
        let mut not = 0;
        let mut xnor = 0;
        let mut nimp = 0;
        let mut nsor = 0;
        for gate in self.1.clone() {
            match gate.name.as_str() {
                "and" => and += 1,
                "or" => or += 1,
                "xor" => xor += 1,
                "nand" => nand += 1,
                "inv" | "not" => not += 1,
                "xnor" => xnor += 1,
                "nimp" => nimp += 1,
                "nsor" => nsor += 1,
                _ => panic!("this gate type is not allowed"),
            }
        }
        GateCount {
            and,
            or,
            xor,
            nand,
            not,
            xnor,
            nimp,
            nsor,
        }
    }
}

#[derive(Clone)]
pub struct TestGate {
    pub wire_a: Rc<RefCell<TestWire>>,
    pub wire_b: Rc<RefCell<TestWire>>,
    pub wire_c: Rc<RefCell<TestWire>>,
    pub name: String,
}

impl TestGate {
    pub fn new(
        wire_a: Rc<RefCell<TestWire>>,
        wire_b: Rc<RefCell<TestWire>>,
        wire_c: Rc<RefCell<TestWire>>,
        name: String,
    ) -> Self {
        Self {
            wire_a,
            wire_b,
            wire_c,
            name,
        }
    }

    pub fn and(
        wire_a: Rc<RefCell<TestWire>>,
        wire_b: Rc<RefCell<TestWire>>,
        wire_c: Rc<RefCell<TestWire>>,
    ) -> Self {
        Self::new(wire_a, wire_b, wire_c, "and".to_string())
    }

    pub fn nand(
        wire_a: Rc<RefCell<TestWire>>,
        wire_b: Rc<RefCell<TestWire>>,
        wire_c: Rc<RefCell<TestWire>>,
    ) -> Self {
        Self::new(wire_a, wire_b, wire_c, "nand".to_string())
    }

    pub fn or(
        wire_a: Rc<RefCell<TestWire>>,
        wire_b: Rc<RefCell<TestWire>>,
        wire_c: Rc<RefCell<TestWire>>,
    ) -> Self {
        Self::new(wire_a, wire_b, wire_c, "or".to_string())
    }

    pub fn xor(
        wire_a: Rc<RefCell<TestWire>>,
        wire_b: Rc<RefCell<TestWire>>,
        wire_c: Rc<RefCell<TestWire>>,
    ) -> Self {
        Self::new(wire_a, wire_b, wire_c, "xor".to_string())
    }

    pub fn xnor(
        wire_a: Rc<RefCell<TestWire>>,
        wire_b: Rc<RefCell<TestWire>>,
        wire_c: Rc<RefCell<TestWire>>,
    ) -> Self {
        Self::new(wire_a, wire_b, wire_c, "xnor".to_string())
    }

    pub fn not(wire_a: Rc<RefCell<TestWire>>, wire_c: Rc<RefCell<TestWire>>) -> Self {
        Self::new(wire_a.clone(), wire_a.clone(), wire_c, "not".to_string())
    }

    pub fn nimp(
        wire_a: Rc<RefCell<TestWire>>,
        wire_b: Rc<RefCell<TestWire>>,
        wire_c: Rc<RefCell<TestWire>>,
    ) -> Self {
        Self::new(wire_a.clone(), wire_b.clone(), wire_c, "nimp".to_string())
    }

    pub fn nsor(
        wire_a: Rc<RefCell<TestWire>>,
        wire_b: Rc<RefCell<TestWire>>,
        wire_c: Rc<RefCell<TestWire>>,
    ) -> Self {
        Self::new(wire_a.clone(), wire_b.clone(), wire_c, "nsor".to_string())
    }

    pub fn f(&self) -> fn(bool, bool) -> bool {
        match self.name.as_str() {
            "and" => {
                fn and(a: bool, b: bool) -> bool {
                    a & b
                }
                and
            }
            "or" => {
                fn or(a: bool, b: bool) -> bool {
                    a | b
                }
                or
            }
            "xor" => {
                fn xor(a: bool, b: bool) -> bool {
                    a ^ b
                }
                xor
            }
            "nand" => {
                fn nand(a: bool, b: bool) -> bool {
                    !(a & b)
                }
                nand
            }
            "inv" | "not" => {
                fn not(a: bool, _b: bool) -> bool {
                    !a
                }
                not
            }
            "xnor" => {
                fn xnor(a: bool, b: bool) -> bool {
                    !(a ^ b)
                }
                xnor
            }
            "nimp" => {
                fn nimp(a: bool, b: bool) -> bool {
                    (a) && (!b)
                }
                nimp
            }
            "nsor" => {
                fn nsor(a: bool, b: bool) -> bool {
                    a | (!b)
                }
                nsor
            }
            _ => {
                panic!("this gate type is not allowed");
            }
        }
    }

    pub fn evaluate(&mut self) {
        self.wire_c.borrow_mut().set((self.f())(
            self.wire_a.borrow().get_value(),
            self.wire_b.borrow().get_value(),
        ));
    }
}

#[derive(Clone, Debug)]
pub struct TestWire {
    pub value: Option<bool>,
}

pub type TestWirex = Rc<RefCell<TestWire>>;
pub type TestWires = Vec<TestWirex>;

impl Default for TestWire {
    fn default() -> Self {
        Self::new()
    }
}

impl TestWire {
    pub fn new() -> Self {
        Self { value: None }
    }

    pub fn get_value(&self) -> bool {
        assert!(self.value.is_some());
        self.value.unwrap()
    }

    pub fn set(&mut self, bit: bool) {
        assert!(self.value.is_none());
        self.value = Some(bit);
    }
}
