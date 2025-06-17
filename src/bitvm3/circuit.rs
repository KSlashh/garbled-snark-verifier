use num_bigint::BigUint;
use num_traits::One;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::bitvm3::math::{mod_bipow_mul, mod_inv, mod_mul};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Label {
    pub flag: bool,
    pub label: Vec<u8>,
}

impl Label {
    pub fn new(flag: bool, label: BigUint) -> Self {
        Label {
            flag,
            label: label.to_bytes_le(),
        }
    }
}

pub type Adaptor = Vec<u8>;

#[derive(Clone, Debug, Serialize, Deserialize, EnumString, Display)]
pub enum GateType {
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Xnot,
    Nimp,
    Nsor,
    Input,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gate {
    pub gate_type: GateType,
    pub inputs: (u64, u64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Circuit {
    pub gates: Vec<Gate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitIO {
    pub inputs_index: Vec<u64>,
    pub outputs_index: Vec<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInstance {
    pub circuit: Circuit,
    pub io: CircuitIO,
    pub inputs: Vec<Label>,
    pub output: Option<Vec<Label>>,
    pub gate_outputs: Vec<Option<Label>>,
}

#[derive(Clone, Debug)]
pub struct PublicParameters {
    pub n: BigUint,
    pub e: BigUint,
    pub e1: BigUint,
    pub e2: BigUint,
    pub e3: BigUint,
    pub e4: BigUint,
}

enum AdaptorMask {
    B0110,
    B0101,
    B0011,
    None,
}

fn get_adaptor_mask(gate_type: GateType) -> AdaptorMask {
    match gate_type {
        GateType::And => AdaptorMask::B0110,
        GateType::Or => AdaptorMask::B0011,
        GateType::Xor => AdaptorMask::B0011,
        GateType::Nand => AdaptorMask::B0110,
        GateType::Nor => AdaptorMask::B0011,
        GateType::Xnot => AdaptorMask::B0011,
        GateType::Nimp => AdaptorMask::B0101,
        GateType::Nsor => AdaptorMask::B0011,
        GateType::Input => AdaptorMask::None,
    }
}

fn label_computation(
    params: &PublicParameters,
    adaptors: &(Adaptor, Adaptor),
    input_flags: (bool, bool),
    input_labels: (BigUint, BigUint),
    adaptor_mask: AdaptorMask,
) -> BigUint {
    let (label_a, label_b) = input_labels;
    match adaptor_mask {
        AdaptorMask::B0110 => match input_flags {
            (false, false) => {
                let label_c00 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e1, &params.n);
                label_c00
            }
            (false, true) => {
                let label_c01 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e2, &params.n);
                let adaptor_0 = BigUint::from_bytes_le(&adaptors.0);
                mod_mul(&label_c01, &adaptor_0, &params.n)
            }
            (true, false) => {
                let label_c10 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e3, &params.n);
                let adaptor_1 = BigUint::from_bytes_le(&adaptors.1);
                mod_mul(&label_c10, &adaptor_1, &params.n)
            }
            (true, true) => {
                let label_c11 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e4, &params.n);
                label_c11
            }
        },
        AdaptorMask::B0011 => match input_flags {
            (false, false) => {
                let label_c00 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e1, &params.n);
                label_c00
            }
            (false, true) => {
                let label_c01 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e2, &params.n);
                label_c01
            }
            (true, false) => {
                let label_c10 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e3, &params.n);
                let adaptor_0 = BigUint::from_bytes_le(&adaptors.0);
                mod_mul(&label_c10, &adaptor_0, &params.n)
            }
            (true, true) => {
                let label_c11 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e4, &params.n);
                let adaptor_1 = BigUint::from_bytes_le(&adaptors.1);
                mod_mul(&label_c11, &adaptor_1, &params.n)
            }
        },
        AdaptorMask::B0101 => match input_flags {
            (false, false) => {
                let label_c00 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e1, &params.n);
                label_c00
            }
            (false, true) => {
                let label_c01 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e2, &params.n);
                let adaptor_0 = BigUint::from_bytes_le(&adaptors.0);
                mod_mul(&label_c01, &adaptor_0, &params.n)
            }
            (true, false) => {
                let label_c10 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e3, &params.n);
                label_c10
            }
            (true, true) => {
                let label_c11 = mod_bipow_mul(&label_a, &params.e, &label_b, &params.e4, &params.n);
                let adaptor_1 = BigUint::from_bytes_le(&adaptors.1);
                mod_mul(&label_c11, &adaptor_1, &params.n)
            }
        },
        AdaptorMask::None => label_a,
    }
}

pub fn evaluate_gate(
    gate_type: &GateType,
    params: &PublicParameters,
    input_labels: (&Label, &Label),
    adaptors: &(Adaptor, Adaptor),
) -> Label {
    let flag_a = input_labels.0.flag;
    let flag_b = input_labels.1.flag;

    let label_a = BigUint::from_bytes_le(&input_labels.0.label);
    let label_b = BigUint::from_bytes_le(&input_labels.1.label);

    let label_c = label_computation(
        params,
        adaptors,
        (flag_a, flag_b),
        (label_a, label_b),
        get_adaptor_mask(gate_type.clone()),
    );

    Label::new(quick_evaluate_gate(gate_type, (flag_a, flag_b)), label_c)
}

pub fn quick_evaluate_gate(gate_type: &GateType, input_flags: (bool, bool)) -> bool {
    let (a, b) = input_flags;
    match gate_type {
        GateType::And => a & b,
        GateType::Or => a | b,
        GateType::Xor => a ^ b,
        GateType::Nand => !(a & b),
        GateType::Nor => !(a | b),
        GateType::Xnot => !(a ^ b),
        GateType::Nimp => a & !b,
        GateType::Nsor => a | !b,
        GateType::Input => a,
    }
}

pub fn garble(
    gate_type: &GateType,
    params: &PublicParameters,
    input_labels: (&(Label, Label), &(Label, Label)),
) -> ((Adaptor, Adaptor), (Label, Label)) {
    let (a0, a1, b0, b1) = (
        BigUint::from_bytes_le(&input_labels.0.0.label),
        BigUint::from_bytes_le(&input_labels.0.1.label),
        BigUint::from_bytes_le(&input_labels.1.0.label),
        BigUint::from_bytes_le(&input_labels.1.1.label),
    );
    let PublicParameters {
        n,
        e,
        e1,
        e2,
        e3,
        e4,
    } = params;
    match gate_type {
        GateType::And => {
            // T0 = c00 / c01 , T1 = c00 / c10
            let c00 = mod_bipow_mul(&a0, &e, &b0, &e1, &n);
            let c01 = mod_bipow_mul(&a0, &e, &b1, &e2, &n);
            let c10 = mod_bipow_mul(&a1, &e, &b0, &e3, &n);
            let c11 = mod_bipow_mul(&a1, &e, &b1, &e4, &n);
            (
                (
                    mod_mul(&c00, &mod_inv(&c01, &n).unwrap(), &n).to_bytes_le(),
                    mod_mul(&c00, &mod_inv(&c10, &n).unwrap(), &n).to_bytes_le(),
                ),
                (Label::new(false, c00), Label::new(true, c11)),
            )
        }
        GateType::Or => {
            // T0 = c01 / c10 , T1 = c01 / c11
            let c00 = mod_bipow_mul(&a0, &e, &b0, &e1, &n);
            let c01 = mod_bipow_mul(&a0, &e, &b1, &e2, &n);
            let c10 = mod_bipow_mul(&a1, &e, &b0, &e3, &n);
            let c11 = mod_bipow_mul(&a1, &e, &b1, &e4, &n);
            (
                (
                    mod_mul(&c01, &mod_inv(&c10, &n).unwrap(), &n).to_bytes_le(),
                    mod_mul(&c01, &mod_inv(&c11, &n).unwrap(), &n).to_bytes_le(),
                ),
                (Label::new(false, c00), Label::new(true, c01)),
            )
        }
        GateType::Xor => {
            // T0 = c01 / c10 , T1 = c00 / c11
            let c00 = mod_bipow_mul(&a0, &e, &b0, &e1, &n);
            let c01 = mod_bipow_mul(&a0, &e, &b1, &e2, &n);
            let c10 = mod_bipow_mul(&a1, &e, &b0, &e3, &n);
            let c11 = mod_bipow_mul(&a1, &e, &b1, &e4, &n);
            (
                (
                    mod_mul(&c01, &mod_inv(&c10, &n).unwrap(), &n).to_bytes_le(),
                    mod_mul(&c00, &mod_inv(&c11, &n).unwrap(), &n).to_bytes_le(),
                ),
                (Label::new(false, c00), Label::new(true, c01)),
            )
        }
        GateType::Nand => {
            // T0 = c00 / c01 , T1 = c00 / c10
            let c00 = mod_bipow_mul(&a0, &e, &b0, &e1, &n);
            let c01 = mod_bipow_mul(&a0, &e, &b1, &e2, &n);
            let c10 = mod_bipow_mul(&a1, &e, &b0, &e3, &n);
            let c11 = mod_bipow_mul(&a1, &e, &b1, &e4, &n);
            (
                (
                    mod_mul(&c00, &mod_inv(&c01, &n).unwrap(), &n).to_bytes_le(),
                    mod_mul(&c00, &mod_inv(&c10, &n).unwrap(), &n).to_bytes_le(),
                ),
                (Label::new(false, c11), Label::new(true, c00)),
            )
        }
        GateType::Nor => {
            // T0 = c01 / c10 , T1 = c01 / c11
            let c00 = mod_bipow_mul(&a0, &e, &b0, &e1, &n);
            let c01 = mod_bipow_mul(&a0, &e, &b1, &e2, &n);
            let c10 = mod_bipow_mul(&a1, &e, &b0, &e3, &n);
            let c11 = mod_bipow_mul(&a1, &e, &b1, &e4, &n);
            (
                (
                    mod_mul(&c01, &mod_inv(&c10, &n).unwrap(), &n).to_bytes_le(),
                    mod_mul(&c01, &mod_inv(&c11, &n).unwrap(), &n).to_bytes_le(),
                ),
                (Label::new(false, c01), Label::new(true, c00)),
            )
        }
        GateType::Xnot => {
            // T0 = c01 / c10 , T1 = c00 / c11
            let c00 = mod_bipow_mul(&a0, &e, &b0, &e1, &n);
            let c01 = mod_bipow_mul(&a0, &e, &b1, &e2, &n);
            let c10 = mod_bipow_mul(&a1, &e, &b0, &e3, &n);
            let c11 = mod_bipow_mul(&a1, &e, &b1, &e4, &n);
            (
                (
                    mod_mul(&c01, &mod_inv(&c10, &n).unwrap(), &n).to_bytes_le(),
                    mod_mul(&c00, &mod_inv(&c11, &n).unwrap(), &n).to_bytes_le(),
                ),
                (Label::new(false, c01), Label::new(true, c00)),
            )
        }
        GateType::Nimp => {
            // T0 = c00 / c01 , T1 = c00 / c11
            let c00 = mod_bipow_mul(&a0, &e, &b0, &e1, &n);
            let c01 = mod_bipow_mul(&a0, &e, &b1, &e2, &n);
            let c10 = mod_bipow_mul(&a1, &e, &b0, &e3, &n);
            let c11 = mod_bipow_mul(&a1, &e, &b1, &e4, &n);
            (
                (
                    mod_mul(&c00, &mod_inv(&c01, &n).unwrap(), &n).to_bytes_le(),
                    mod_mul(&c00, &mod_inv(&c11, &n).unwrap(), &n).to_bytes_le(),
                ),
                (Label::new(false, c00), Label::new(true, c10)),
            )
        }
        GateType::Nsor => {
            // T0 = c00 / c10 , T1 = c00 / c11
            let c00 = mod_bipow_mul(&a0, &e, &b0, &e1, &n);
            let c01 = mod_bipow_mul(&a0, &e, &b1, &e2, &n);
            let c10 = mod_bipow_mul(&a1, &e, &b0, &e3, &n);
            let c11 = mod_bipow_mul(&a1, &e, &b1, &e4, &n);
            (
                (
                    mod_mul(&c00, &mod_inv(&c10, &n).unwrap(), &n).to_bytes_le(),
                    mod_mul(&c00, &mod_inv(&c11, &n).unwrap(), &n).to_bytes_le(),
                ),
                (Label::new(false, c01), Label::new(true, c00)),
            )
        }
        GateType::Input => (
            (BigUint::one().to_bytes_le(), BigUint::one().to_bytes_le()),
            (Label::new(false, a0), Label::new(true, a1)),
        ),
    }
}

mod test {
    #![allow(unused_assignments, dead_code)]
    use num_traits::FromPrimitive;

    use crate::bitvm3::math::random_biguint;

    use super::*;

    #[macro_export]
    macro_rules! gate {
        ($gate_type:ident, $in1:expr, $in2:expr) => {
            Gate {
                gate_type: GateType::$gate_type,
                inputs: ($in1, $in2),
            }
        };
        ($gate_type:ident) => {
            Gate {
                gate_type: GateType::$gate_type,
                inputs: (0, 0),
            }
        };
    }

    #[macro_export]
    macro_rules! circuit {
        (
            $( $gate_type:ident $( ( $in1:expr, $in2:expr ) )? $( : $tag:ident )? ),* $(,)?
        ) => {{
            let mut gates = Vec::new();
            let mut input_indices = Vec::new();
            let mut output_indices = Vec::new();
            let mut idx = 0;

            $(
                let gate = gate!(
                    $gate_type
                    $(, $in1, $in2)?
                );

                if matches!(gate.gate_type, GateType::Input) {
                    input_indices.push(idx);
                }

                gates.push(gate);

                circuit!(@push_tag idx, output_indices, $($tag)?);

                idx += 1;
            )*

            (
                Circuit { gates },
                CircuitIO {
                    inputs_index: input_indices,
                    outputs_index: output_indices,
                }
            )
        }};

        (@push_tag $idx:ident, $vec:ident, output) => {
            $vec.push($idx);
        };

        (@push_tag $idx:ident, $vec:ident, ) => {};
    }

    fn gen_test_circuit() -> (Circuit, CircuitIO) {
        // use expample circuit in https://github.com/delbrag/delbrag/blob/main/README.md
        circuit![
            Input, // 0: in00
            Input, // 1: in10
            Input, // 2: in01
            Input, // 3: in11
            Xor(0, 1):output, // 4: G0 out0
            And(0, 1), // 5: G1 m0
            Xor(2, 3), // 6: G2 m1
            And(2, 3), // 7: G3 m2
            Xor(5, 6):output, // 8: G4 out1
            And(5, 6), // 9: G5 m3
            Or(9, 3):output, // 10: G6 out2
        ]
    }

    fn gen_test_params() -> PublicParameters {
        PublicParameters {
            // p = 0xe5a111a219c64f841669400f51a54dd4e75184004f0f4d21c6ae182cfb528652a02d6d677a72b564c505b1ed42a0c648dbfe14eb66b04c0d60ba3872826c32e7
            // q = 0x98cb760764484e29245521be08e7f38edeebfca8427149524ba7f4735e1d5f3a45d585cb3722ff4c07c19165be738311dc346a914966f5b311416fed3b425079
            n: BigUint::parse_bytes(b"890e23101a542913da8a4350672c9ef8e7b34c2687ce8cd8db3fb34244a791d60c9dc0a53172a56dcc8a66f553c0ae51e9e2e2ce9486fa6b00a6c556bfed139001133cdfe5921c425eb8823b1bd0a4c00920d24bee2633256328502eadbfac1420f9a5f47139de6f14d8eb7c2b7c0cec42530c0a71dadb80c7214f5cd19a3f2f", 16).unwrap(),
            e: BigUint::from_u8(3).unwrap(),
            e1: BigUint::from_u8(5).unwrap(),
            e2: BigUint::from_u8(7).unwrap(),
            e3: BigUint::from_u8(11).unwrap(),
            e4: BigUint::from_u8(13).unwrap(),
        }
    }

    fn random_label_pair() -> (Label, Label) {
        (
            Label {
                flag: false,
                label: random_biguint().to_bytes_le(),
            },
            Label {
                flag: true,
                label: random_biguint().to_bytes_le(),
            },
        )
    }

    #[test]
    fn test_compile_and_evaluate() {
        // setup
        println!("1. setup");
        let (circuit, io) = gen_test_circuit();
        let params = gen_test_params();

        let input_0 = random_label_pair();
        let input_1 = random_label_pair();
        let input_2 = random_label_pair();
        let input_3 = random_label_pair();
        let mut labels = vec![input_0.clone(), input_1.clone(), input_2.clone(), input_3.clone()];

        // garbler compute adaptors & labels
        println!("2. garbler compute adaptors & labels");
        let mut adaptors: Vec<(Adaptor, Adaptor)> = vec![];
        for _ in 0..io.inputs_index.len() {
            adaptors.push((vec![],vec![]));
        }
        for i in io.inputs_index.len()..circuit.gates.len() {
            let gate = &circuit.gates[i];
            let input_labels = (
                &labels[gate.inputs.0 as usize],
                &labels[gate.inputs.1 as usize],
            );
            let (gate_adaptors, gate_labels) = garble(&gate.gate_type, &params, input_labels);
            adaptors.push(gate_adaptors);
            labels.push(gate_labels);
        }

        // evaluate circuit: 0x10 + 0x11 = 0x101 (2+3=5)
        let public_input_labels = vec![input_0.1, input_1.0, input_2.1, input_3.1];
        let expected_output_labels = vec![labels[4].1.clone(), labels[8].0.clone(), labels[10].1.clone()];

        // verifier compute labels
        println!("3. verifier compute labels");
        let mut verifier_labels = public_input_labels;
        for i in io.inputs_index.len()..circuit.gates.len() {
            let gate = &circuit.gates[i];
            let input_labels = (
                &verifier_labels[gate.inputs.0 as usize],
                &verifier_labels[gate.inputs.1 as usize],
            );
            let gate_label = evaluate_gate(&gate.gate_type, &params, input_labels, &adaptors[i]);
            verifier_labels.push(gate_label);
        } 
        
        println!("4. compare output labels");
        let mut verifier_ouput_labels = vec![];
        for i in io.outputs_index {
            verifier_ouput_labels.push(verifier_labels[i as usize].clone());
        }
        assert_eq!(expected_output_labels, verifier_ouput_labels);
    }
}
