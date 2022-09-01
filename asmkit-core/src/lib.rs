//! The core behind AsmKit.

use entity::LabelRef;

pub mod entity;

/// The output of an instruction stream.  Keeps tracks of any relocations.
/// 
/// TODO: implement relocations.
#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    /// The bytes produce by an instruction stream.
    bytes: Vec<u8>,
}

impl Product {
    /// Creates a new product initialized with the provided bytes.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Finalizes the instruction stream output and returns the produced bytes.
    pub fn emit(self) -> Vec<u8> {
        self.bytes
    }
}

/// A target-specific stream of instructions.
/// 
/// An instruction stream inputs instructions and immediately encodes them into machine code for its respective target architecture.  Relocation is performed later.
pub trait InstructionStream {
    /// Creates a new label.
    /// 
    /// The label will be created without being attached to any index.  If the label is used before it is attached, it will be used as a relocation rather than
    /// pre-calculating the offset.
    fn create_label(&mut self) -> LabelRef;

    /// Creates a new label and attaches it to the current index.
    /// 
    /// When the label is used, AsmKit will automatically calculate the offset of the label instead of creating a relocation.
    fn create_label_attached(&mut self) -> LabelRef;

    /// Attaches a label to the current index.
    /// 
    /// After attaching, AsmKit will automatically calculate the offset of the label instead of creating an unnecessary relocation for it.
    /// 
    /// This will overwrite any previous label at the provided reference.
    fn attach_label(&mut self, label: LabelRef);

    /// Writes a raw byte into the instruction stream at the current index.
    fn write_byte(&mut self, byte: u8);

    /// Writes a raw word into the instruction stream at the current index.
    /// 
    /// The value will be swapped into the endianness of the target, by the implementor of [`InstructionStream`].
    fn write_word(&mut self, word: u16);

    /// Writes a raw double word into the instruction stream at the current index.
    /// 
    /// The value will be swapped into the endianness of the target, by the implementor of [`InstructionStream`].
    fn write_double_word(&mut self, word: u32);

    /// Writes a raw quadruple word into the instruction stream at the current index.
    /// 
    /// The value will be swapped into the endianness of the target, by the implementor of [`InstructionStream`].
    fn write_quad_word(&mut self, word: u64);

    /// Writes a raw double quadruple word into the instruction stream at the current index.
    /// 
    /// The value will be swapped into the endianness of the target, by the implementor of [`InstructionStream`].
    fn write_double_quad_word(&mut self, word: u128);

    /// Finishes writing to the instruction stream and returns the produced bytes.
    fn finish(self) -> Product;
}