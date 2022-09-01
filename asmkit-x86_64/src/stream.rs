//! Instruction streaming implementation.

use asmkit_core::{entity::{EntityList, Label, LabelRef}, InstructionStream, Product};

use crate::register::{Reg64, Reg8, Reg16, Reg32};

pub const REX: u8 = 0b01000000;
pub const REX_W: u8 = 0b1000;
pub const REX_R: u8 = 0b100;
pub const REX_X: u8 = 0b10;
pub const REX_B: u8 = 0b1;

/// An instruction stream for the x86_64 target.
#[allow(non_camel_case_types)]
pub struct x86_64InstructionStream {
    /// The bytes written to the instruction stream.
    bytes: Vec<u8>,

    /// The labels written to the instruction stream.
    labels: EntityList<Label, LabelRef>,
}

impl InstructionStream for x86_64InstructionStream {
    #[inline(always)]
    fn create_label(&mut self) -> LabelRef {
        self.labels.push(Label::Unattached)
    }

    #[inline(always)]
    fn create_label_attached(&mut self) -> LabelRef {
        self.labels.push(Label::Attached(self.labels.len()))
    }

    #[inline(always)]
    fn attach_label(&mut self, label: LabelRef) {
        *self.labels.get_mut(label) = Label::Attached(self.labels.len());
    }

    #[inline(always)]
    fn write_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    #[inline(always)]
    fn write_word(&mut self, word: u16) {
        self.bytes.append(&mut word.to_le_bytes().into_iter().collect());
    }

    #[inline(always)]
    fn write_double_word(&mut self, word: u32) {
        self.bytes.append(&mut word.to_le_bytes().into_iter().collect());
    }

    #[inline(always)]
    fn write_quad_word(&mut self, word: u64) {
        self.bytes.append(&mut word.to_le_bytes().into_iter().collect());
    }

    #[inline(always)]
    fn write_double_quad_word(&mut self, word: u128) {
        self.bytes.append(&mut word.to_le_bytes().into_iter().collect());
    }

    #[inline(always)]
    fn finish(self) -> Product {
        Product::new(self.bytes)
    }
}

impl x86_64InstructionStream {
    /// Creates a new, empty instruction stream.
    #[inline(always)]
    pub fn new() -> Self {
        Self { bytes: Vec::new(), labels: EntityList::new() }
    }

    /// Move `src` to `dest.
    pub fn mov_reg8_reg8(&mut self, dest: Reg8, src: Reg8) {
        let is_dest_extension = dest.is_extension();
        let is_src_extension = src.is_extension();
        let is_dest_reserved = dest.is_reserved();
        let is_src_reserved = src.is_reserved();
        
        if is_dest_extension | is_src_extension | is_dest_reserved | is_src_reserved {
            let mut prefix = REX;

            if is_dest_extension || is_dest_reserved {
                prefix |= REX_B;
            }
    
            if is_src_extension || is_src_reserved {
                prefix |= REX_R;
            }

            self.write_byte(prefix); // REX prefix
        }

        self.write_byte(88); // opcode
        self.write_byte((0b11 << 6) | (src.offset() << 3) | (dest.offset())); // value MODRM encoded
    }

    /// Move `src` to `dest.
    pub fn mov_reg16_reg16(&mut self, dest: Reg16, src: Reg16) {
        self.write_byte(0x66);

        let is_dest_extension = dest.is_extension();
        let is_src_extension = src.is_extension();
        if is_dest_extension || is_src_extension {
            let mut prefix = REX;

            if is_dest_extension {
                prefix |= REX_B;
            }
    
            if is_src_extension {
                prefix |= REX_R;
            }

            self.write_byte(prefix);
        }

        self.write_byte(0x89);
        self.write_byte((0b11 << 6) | (src.offset() << 3) | (dest.offset()));
    }

    /// Move `src` to `dest.
    pub fn mov_reg32_reg32(&mut self, dest: Reg32, src: Reg32) {
        let is_dest_extension = dest.is_extension();
        let is_src_extension = src.is_extension();
        if is_dest_extension || is_src_extension {
            let mut prefix = REX;

            if is_dest_extension {
                prefix |= REX_B;
            }
    
            if is_src_extension {
                prefix |= REX_R;
            }

            self.write_byte(prefix);
        }

        self.write_byte(0x89);
        self.write_byte((0b11 << 6) | (src.offset() << 3) | (dest.offset()));
    }

    /// Move `src` to `dest.
    pub fn mov_reg64_reg64(&mut self, dest: Reg64, src: Reg64) {
        let mut prefix = REX | REX_W; // REX.W prefix

        if dest.is_extension() {
            prefix |= REX_B;
        }

        if src.is_extension() {
            prefix |= REX_R;
        }

        self.write_byte(prefix);
        self.write_byte(0x89); // opcode
        self.write_byte((0b11 << 6) | (src.offset() << 3) | (dest.offset())); // value MODRM encoded
    }

    /// Move `src` to `dest.
    pub fn mov_reg8_imm8(&mut self, dest: Reg8, src: u8) {
        if dest.is_extension() {
            self.write_byte(REX | REX_B);
        } else if dest.is_reserved() {
            self.write_byte(REX);
        }

        self.write_byte(0xb0 + dest.offset()); // opcode
        self.write_byte(src);
    }

    /// Move `src` to `dest.
    pub fn mov_reg16_imm16(&mut self, dest: Reg16, src: u16) {
        self.write_byte(0x66); // prefix

        if dest.is_extension() {
            self.write_byte(REX | REX_B); // extension prefix
        }

        self.write_byte(0xb8 + dest.offset()); // opcode
        self.write_word(src);
    }

    /// Move `src` to `dest.
    pub fn mov_reg32_imm32(&mut self, dest: Reg32, src: u32) {
        if dest.is_extension() {
            self.write_byte(REX | REX_B); // extension prefix
        }

        self.write_byte(0xb8 + dest.offset()); // opcode
        self.write_double_word(src);
    }

    /// Move `src` to `dest.
    pub fn mov_reg64_imm32(&mut self, dest: Reg64, src: u32) {
        let mut prefix = REX | REX_W; // REX.W prefix

        if dest.is_extension() {
            prefix = prefix | REX_B;
        }

        self.write_byte(prefix);
        self.write_byte(0xc7 + dest.offset()); // opcode
        self.write_double_word(src);
    }

    /// Move `src` to `dest.
    pub fn mov_reg64_imm64(&mut self, dest: Reg64, src: u64) {
        // REX prefix
        let mut prefix = REX | REX_W;

        if dest.is_extension() {
            prefix |= REX_B;
        }

        self.write_byte(prefix);
        self.write_byte(0xb8 + dest.offset()); // opcode
        self.write_quad_word(src);
    }

    /// Returns from the current function.
    #[inline(always)]
    pub fn ret(&mut self) {
        self.write_byte(0xc3); // opcode
    }
}