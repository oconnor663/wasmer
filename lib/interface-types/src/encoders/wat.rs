//! Writes the AST into a string representing WIT with its textual format.
//!
//! # Example
//!
//! ```rust
//! use wasmer_interface_types::{
//!     ast::*,
//!     encoders::wat::*,
//!     interpreter::Instruction,
//! };
//!
//! let input: String = (&Interfaces {
//!     types: vec![Type {
//!         inputs: vec![InterfaceType::I32],
//!         outputs: vec![InterfaceType::S8],
//!     }],
//!     imports: vec![Import {
//!         namespace: "ns",
//!         name: "foo",
//!         signature_type: 0,
//!     }],
//!     adapters: vec![Adapter {
//!         function_type: 0,
//!         instructions: vec![Instruction::ArgumentGet { index: 42 }],
//!     }],
//!     exports: vec![Export {
//!         name: "bar",
//!         function_type: 0,
//!     }],
//!     implementations: vec![Implementation {
//!         core_function_type: 0,
//!         adapter_function_type: 1,
//!     }],
//! })
//!     .to_string();
//! let output = r#";; Types
//! (@interface type (func
//!   (param i32)
//!   (result s8)))
//!
//! ;; Imports
//! (@interface import "ns" "foo" (func (type 0)))
//!
//! ;; Adapters
//! (@interface func (type 0)
//!   arg.get 42)
//!
//! ;; Exports
//! (@interface export "bar" (func 0))
//!
//! ;; Implementations
//! (@interface implement (func 0) (func 1))"#;
//!
//! assert_eq!(input, output);
//! ```

use crate::{ast::*, interpreter::Instruction};
use std::string::ToString;

/// Encode an `InterfaceType` into a string.
impl ToString for &InterfaceType {
    fn to_string(&self) -> String {
        match self {
            InterfaceType::S8 => "s8".into(),
            InterfaceType::S16 => "s16".into(),
            InterfaceType::S32 => "s32".into(),
            InterfaceType::S64 => "s64".into(),
            InterfaceType::U8 => "u8".into(),
            InterfaceType::U16 => "u16".into(),
            InterfaceType::U32 => "u32".into(),
            InterfaceType::U64 => "u64".into(),
            InterfaceType::F32 => "f32".into(),
            InterfaceType::F64 => "f64".into(),
            InterfaceType::String => "string".into(),
            InterfaceType::Anyref => "anyref".into(),
            InterfaceType::I32 => "i32".into(),
            InterfaceType::I64 => "i64".into(),
        }
    }
}

/// Encode an `Instruction` into a string.
impl ToString for &Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::ArgumentGet { index } => format!("arg.get {}", index),
            Instruction::CallCore { function_index } => format!("call-core {}", function_index),
            Instruction::S8FromI32 => "s8.from_i32".into(),
            Instruction::S8FromI64 => "s8.from_i64".into(),
            Instruction::S16FromI32 => "s16.from_i32".into(),
            Instruction::S16FromI64 => "s16.from_i64".into(),
            Instruction::S32FromI32 => "s32.from_i32".into(),
            Instruction::S32FromI64 => "s32.from_i64".into(),
            Instruction::S64FromI32 => "s64.from_i32".into(),
            Instruction::S64FromI64 => "s64.from_i64".into(),
            Instruction::I32FromS8 => "i32.from_s8".into(),
            Instruction::I32FromS16 => "i32.from_s16".into(),
            Instruction::I32FromS32 => "i32.from_s32".into(),
            Instruction::I32FromS64 => "i32.from_s64".into(),
            Instruction::I64FromS8 => "i64.from_s8".into(),
            Instruction::I64FromS16 => "i64.from_s16".into(),
            Instruction::I64FromS32 => "i64.from_s32".into(),
            Instruction::I64FromS64 => "i64.from_s64".into(),
            Instruction::U8FromI32 => "u8.from_i32".into(),
            Instruction::U8FromI64 => "u8.from_i64".into(),
            Instruction::U16FromI32 => "u16.from_i32".into(),
            Instruction::U16FromI64 => "u16.from_i64".into(),
            Instruction::U32FromI32 => "u32.from_i32".into(),
            Instruction::U32FromI64 => "u32.from_i64".into(),
            Instruction::U64FromI32 => "u64.from_i32".into(),
            Instruction::U64FromI64 => "u64.from_i64".into(),
            Instruction::I32FromU8 => "i32.from_u8".into(),
            Instruction::I32FromU16 => "i32.from_u16".into(),
            Instruction::I32FromU32 => "i32.from_u32".into(),
            Instruction::I32FromU64 => "i32.from_u64".into(),
            Instruction::I64FromU8 => "i64.from_u8".into(),
            Instruction::I64FromU16 => "i64.from_u16".into(),
            Instruction::I64FromU32 => "i64.from_u32".into(),
            Instruction::I64FromU64 => "i64.from_u64".into(),
            Instruction::StringLiftMemory => "string.lift_memory".into(),
            Instruction::StringLowerMemory { allocator_index } => {
                format!(r#"string.lower_memory {}"#, allocator_index)
            }
            Instruction::StringSize => "string.size".into(),
        }
    }
}

/// Encode a list of `InterfaceType` representing inputs into a
/// string.
fn input_types_to_param(input_types: &[InterfaceType]) -> String {
    if input_types.is_empty() {
        "".into()
    } else {
        format!(
            "\n  (param{})",
            input_types
                .iter()
                .fold(String::new(), |mut accumulator, interface_type| {
                    accumulator.push(' ');
                    accumulator.push_str(&interface_type.to_string());
                    accumulator
                })
        )
    }
}

/// Encode a list of `InterfaceType` representing outputs into a
/// string.
fn output_types_to_result(output_types: &[InterfaceType]) -> String {
    if output_types.is_empty() {
        "".into()
    } else {
        format!(
            "\n  (result{})",
            output_types
                .iter()
                .fold(String::new(), |mut accumulator, interface_type| {
                    accumulator.push(' ');
                    accumulator.push_str(&interface_type.to_string());
                    accumulator
                })
        )
    }
}

/// Encode a `Type` into a string.
impl<'input> ToString for &Type {
    fn to_string(&self) -> String {
        format!(
            r#"(@interface type (func{inputs}{outputs}))"#,
            inputs = input_types_to_param(&self.inputs),
            outputs = output_types_to_result(&self.outputs),
        )
    }
}

/// Encode an `Import` into a string.
impl<'input> ToString for &Import<'input> {
    fn to_string(&self) -> String {
        format!(
            r#"(@interface import "{namespace}" "{name}" (func (type {type})))"#,
            namespace = self.namespace,
            name = self.name,
            type = self.signature_type,
        )
    }
}

/// Encode an `Adapter` into a string.
impl ToString for &Adapter {
    fn to_string(&self) -> String {
        format!(
            r#"(@interface func (type {function_type}){instructions})"#,
            function_type = self.function_type,
            instructions =
                self.instructions
                    .iter()
                    .fold(String::new(), |mut accumulator, instruction| {
                        accumulator.push_str("\n  ");
                        accumulator.push_str(&instruction.to_string());
                        accumulator
                    }),
        )
    }
}

/// Encode an `Export` into a string.
impl<'input> ToString for &Export<'input> {
    fn to_string(&self) -> String {
        format!(
            r#"(@interface export "{name}" (func {type}))"#,
            name = self.name,
            type = self.function_type,
        )
    }
}

/// Encode an `Implementation` into a string.
impl<'input> ToString for &Implementation {
    fn to_string(&self) -> String {
        format!(
            r#"(@interface implement (func {core_function_type}) (func {adapter_function_type}))"#,
            core_function_type = self.core_function_type,
            adapter_function_type = self.adapter_function_type,
        )
    }
}

/// Encode an `Interfaces` into a string.
impl<'input> ToString for &Interfaces<'input> {
    fn to_string(&self) -> String {
        let mut output = String::new();

        let types = self
            .types
            .iter()
            .fold(String::new(), |mut accumulator, ty| {
                accumulator.push('\n');
                accumulator.push_str(&ty.to_string());
                accumulator
            });

        let imports = self
            .imports
            .iter()
            .fold(String::new(), |mut accumulator, import| {
                accumulator.push('\n');
                accumulator.push_str(&import.to_string());
                accumulator
            });

        let adapters = self
            .adapters
            .iter()
            .fold(String::new(), |mut accumulator, adapter| {
                accumulator.push('\n');
                accumulator.push_str(&adapter.to_string());
                accumulator
            });

        let exports = self
            .exports
            .iter()
            .fold(String::new(), |mut accumulator, export| {
                accumulator.push('\n');
                accumulator.push_str(&export.to_string());
                accumulator
            });

        let implementations =
            self.implementations
                .iter()
                .fold(String::new(), |mut accumulator, implementation| {
                    accumulator.push('\n');
                    accumulator.push_str(&implementation.to_string());
                    accumulator
                });

        let separator = |output: &mut String| {
            if !output.is_empty() {
                output.push_str("\n\n");
            }
        };

        if !types.is_empty() {
            output.push_str(";; Types");
            output.push_str(&types);
        }

        separator(&mut output);

        if !imports.is_empty() {
            output.push_str(";; Imports");
            output.push_str(&imports);
        }

        separator(&mut output);

        if !adapters.is_empty() {
            output.push_str(";; Adapters");
            output.push_str(&adapters);
        }

        separator(&mut output);

        if !exports.is_empty() {
            output.push_str(";; Exports");
            output.push_str(&exports);
        }

        separator(&mut output);

        if !implementations.is_empty() {
            output.push_str(";; Implementations");
            output.push_str(&implementations);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::*, interpreter::Instruction};

    #[test]
    fn test_interface_types() {
        let inputs: Vec<String> = vec![
            (&InterfaceType::S8).to_string(),
            (&InterfaceType::S16).to_string(),
            (&InterfaceType::S32).to_string(),
            (&InterfaceType::S64).to_string(),
            (&InterfaceType::U8).to_string(),
            (&InterfaceType::U16).to_string(),
            (&InterfaceType::U32).to_string(),
            (&InterfaceType::U64).to_string(),
            (&InterfaceType::F32).to_string(),
            (&InterfaceType::F64).to_string(),
            (&InterfaceType::String).to_string(),
            (&InterfaceType::Anyref).to_string(),
            (&InterfaceType::I32).to_string(),
            (&InterfaceType::I64).to_string(),
        ];
        let outputs = vec![
            "s8", "s16", "s32", "s64", "u8", "u16", "u32", "u64", "f32", "f64", "string", "anyref",
            "i32", "i64",
        ];

        assert_eq!(inputs, outputs);
    }

    #[test]
    fn test_instructions() {
        let inputs: Vec<String> = vec![
            (&Instruction::ArgumentGet { index: 7 }).to_string(),
            (&Instruction::CallCore { function_index: 7 }).to_string(),
            (&Instruction::S8FromI32).to_string(),
            (&Instruction::S8FromI64).to_string(),
            (&Instruction::S16FromI32).to_string(),
            (&Instruction::S16FromI64).to_string(),
            (&Instruction::S32FromI32).to_string(),
            (&Instruction::S32FromI64).to_string(),
            (&Instruction::S64FromI32).to_string(),
            (&Instruction::S64FromI64).to_string(),
            (&Instruction::I32FromS8).to_string(),
            (&Instruction::I32FromS16).to_string(),
            (&Instruction::I32FromS32).to_string(),
            (&Instruction::I32FromS64).to_string(),
            (&Instruction::I64FromS8).to_string(),
            (&Instruction::I64FromS16).to_string(),
            (&Instruction::I64FromS32).to_string(),
            (&Instruction::I64FromS64).to_string(),
            (&Instruction::U8FromI32).to_string(),
            (&Instruction::U8FromI64).to_string(),
            (&Instruction::U16FromI32).to_string(),
            (&Instruction::U16FromI64).to_string(),
            (&Instruction::U32FromI32).to_string(),
            (&Instruction::U32FromI64).to_string(),
            (&Instruction::U64FromI32).to_string(),
            (&Instruction::U64FromI64).to_string(),
            (&Instruction::I32FromU8).to_string(),
            (&Instruction::I32FromU16).to_string(),
            (&Instruction::I32FromU32).to_string(),
            (&Instruction::I32FromU64).to_string(),
            (&Instruction::I64FromU8).to_string(),
            (&Instruction::I64FromU16).to_string(),
            (&Instruction::I64FromU32).to_string(),
            (&Instruction::I64FromU64).to_string(),
            (&Instruction::StringLiftMemory).to_string(),
            (&Instruction::StringLowerMemory {
                allocator_index: 42,
            })
                .to_string(),
            (&Instruction::StringSize).to_string(),
        ];
        let outputs = vec![
            "arg.get 7",
            "call-core 7",
            "s8.from_i32",
            "s8.from_i64",
            "s16.from_i32",
            "s16.from_i64",
            "s32.from_i32",
            "s32.from_i64",
            "s64.from_i32",
            "s64.from_i64",
            "i32.from_s8",
            "i32.from_s16",
            "i32.from_s32",
            "i32.from_s64",
            "i64.from_s8",
            "i64.from_s16",
            "i64.from_s32",
            "i64.from_s64",
            "u8.from_i32",
            "u8.from_i64",
            "u16.from_i32",
            "u16.from_i64",
            "u32.from_i32",
            "u32.from_i64",
            "u64.from_i32",
            "u64.from_i64",
            "i32.from_u8",
            "i32.from_u16",
            "i32.from_u32",
            "i32.from_u64",
            "i64.from_u8",
            "i64.from_u16",
            "i64.from_u32",
            "i64.from_u64",
            "string.lift_memory",
            "string.lower_memory 42",
            "string.size",
        ];

        assert_eq!(inputs, outputs);
    }

    #[test]
    fn test_types() {
        let inputs: Vec<String> = vec![
            (&Type {
                inputs: vec![InterfaceType::I32, InterfaceType::F32],
                outputs: vec![InterfaceType::I32],
            })
                .to_string(),
            (&Type {
                inputs: vec![InterfaceType::I32],
                outputs: vec![],
            })
                .to_string(),
            (&Type {
                inputs: vec![],
                outputs: vec![InterfaceType::I32],
            })
                .to_string(),
            (&Type {
                inputs: vec![],
                outputs: vec![],
            })
                .to_string(),
        ];
        let outputs = vec![
            r#"(@interface type (func
  (param i32 f32)
  (result i32)))"#,
            r#"(@interface type (func
  (param i32)))"#,
            r#"(@interface type (func
  (result i32)))"#,
            r#"(@interface type (func))"#,
        ];

        assert_eq!(inputs, outputs);
    }

    #[test]
    fn test_exports() {
        let input = (&Export {
            name: "foo",
            function_type: 0,
        })
            .to_string();
        let output = r#"(@interface export "foo" (func 0))"#;

        assert_eq!(input, output);
    }

    #[test]
    fn test_imports() {
        let input = (&Import {
            namespace: "ns",
            name: "foo",
            signature_type: 0,
        })
            .to_string();
        let output = r#"(@interface import "ns" "foo" (func (type 0)))"#;

        assert_eq!(input, output);
    }

    #[test]
    fn test_adapter() {
        let input = (&Adapter {
            function_type: 0,
            instructions: vec![Instruction::ArgumentGet { index: 42 }],
        })
            .to_string();
        let output = r#"(@interface func (type 0)
  arg.get 42)"#;

        assert_eq!(input, output);
    }

    #[test]
    fn test_interfaces() {
        let input: String = (&Interfaces {
            types: vec![Type {
                inputs: vec![InterfaceType::I32],
                outputs: vec![InterfaceType::S8],
            }],
            imports: vec![Import {
                namespace: "ns",
                name: "foo",
                signature_type: 0,
            }],
            adapters: vec![Adapter {
                function_type: 0,
                instructions: vec![Instruction::ArgumentGet { index: 42 }],
            }],
            exports: vec![Export {
                name: "bar",
                function_type: 0,
            }],
            implementations: vec![Implementation {
                core_function_type: 0,
                adapter_function_type: 1,
            }],
        })
            .to_string();
        let output = r#";; Types
(@interface type (func
  (param i32)
  (result s8)))

;; Imports
(@interface import "ns" "foo" (func (type 0)))

;; Adapters
(@interface func (type 0)
  arg.get 42)

;; Exports
(@interface export "bar" (func 0))

;; Implementations
(@interface implement (func 0) (func 1))"#;

        assert_eq!(input, output);
    }
}
