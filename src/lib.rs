pub mod circuits;
pub mod core;

pub mod core_bag {
    pub use crate::core::circuit::Circuit;
    pub use crate::core::gate::Gate;
    pub use crate::core::s::S;
    pub use crate::core::wire::Wire;
    pub use std::{cell::RefCell, rc::Rc};
    pub type Wirex = Rc<RefCell<Wire>>;
    pub type Wires = Vec<Wirex>;
    pub use crate::core::gate::GateCount;
}

pub mod bag {
    pub use std::{cell::RefCell, rc::Rc};
    pub use crate::core::lit_circuit::TestCircuit as Circuit;
    pub use crate::core::lit_circuit::TestGate as Gate;
    pub use crate::core::lit_circuit::TestWirex as Wirex;
    pub use crate::core::lit_circuit::TestWires as Wires;
    pub use crate::core::lit_circuit::TestWire as Wire;
    pub use crate::core::gate::GateCount;
}
