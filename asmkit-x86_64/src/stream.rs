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
        self.bytes.append(&mut word.to_le_bytes().to_vec());
    }

    #[inline(always)]
    fn write_double_word(&mut self, word: u32) {
        self.bytes.append(&mut word.to_le_bytes().to_vec());
    }

    #[inline(always)]
    fn write_quad_word(&mut self, word: u64) {
        self.bytes.append(&mut word.to_le_bytes().to_vec());
    }

    #[inline(always)]
    fn write_double_quad_word(&mut self, word: u128) {
        self.bytes.append(&mut word.to_le_bytes().to_vec());
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

    /// Pushes the stack base pointer onto the stack.
    #[inline(always)]
    pub fn intrinsic_push_rbp(&mut self) {
        self.push_reg64(Reg64::Rbp);
    }

    /// Move *r8* to *r/m8*.
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

    /// Move *r16* to *r/m16*.
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

    /// Move *r32* to *r/m32*.
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

    /// Move *r64* to *r/m64*.
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

    /// Move *imm8* to *r8*.
    pub fn mov_reg8_imm8(&mut self, dest: Reg8, src: u8) {
        if dest.is_extension() {
            self.write_byte(REX | REX_B);
        } else if dest.is_reserved() {
            self.write_byte(REX);
        }

        self.write_byte(0xb0 + dest.offset()); // opcode
        self.write_byte(src);
    }

    /// Move *imm16* to *r16*.
    pub fn mov_reg16_imm16(&mut self, dest: Reg16, src: u16) {
        self.write_byte(0x66); // prefix

        if dest.is_extension() {
            self.write_byte(REX | REX_B); // extension prefix
        }

        self.write_byte(0xb8 + dest.offset()); // opcode
        self.write_word(src);
    }

    /// Move *imm32* to *r32*.
    pub fn mov_reg32_imm32(&mut self, dest: Reg32, src: u32) {
        if dest.is_extension() {
            self.write_byte(REX | REX_B); // extension prefix
        }

        self.write_byte(0xb8 + dest.offset()); // opcode
        self.write_double_word(src);
    }

    /// Move *imm32* to *r64*.
    pub fn mov_reg64_imm32(&mut self, dest: Reg64, src: u32) {
        let mut prefix = REX | REX_W; // REX.W prefix

        if dest.is_extension() {
            prefix = prefix | REX_B;
        }

        self.write_byte(prefix);
        self.write_byte(0xc7 + dest.offset()); // opcode
        self.write_double_word(src);
    }

    /// Move *imm64* to *r64*.
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

    /// Push *r/m16*.
    pub fn push_reg16(&mut self, reg16: Reg16) {
        self.write_byte(REX | REX_X); // rex prefix
        self.write_byte(0x50 + reg16.offset()); // opcode
    }

    /// Push *r/m64*.
    pub fn push_reg64(&mut self, reg64: Reg64) {
        if reg64.is_extension() {
            self.write_byte(0x41); // prefix
        }
        self.write_byte(0x50 + reg64.offset()); // opcode
    }

    /// Push *imm8*.
    pub fn push_imm8(&mut self, imm8: u8) {
        self.write_byte(0x6a);
        self.write_byte(imm8);
    }

    /// Push *imm16*.
    /// 
    /// **NOTE:** extends *imm16* into an *imm32*.
    pub fn push_imm16(&mut self, imm16: u16) {
        self.write_byte(0x68);
        self.write_double_word(imm16 as u32);
    }

    /// Push *imm32*.
    pub fn push_imm32(&mut self, imm32: u32) {
        self.write_byte(0x68);
        self.write_double_word(imm32);
    }

    /// Push FS.
    pub fn push_fs(&mut self) {
        self.write_byte(0x0f);
        self.write_byte(0xa0);
    }

    /// Push GS.
    pub fn push_gs(&mut self) {
        self.write_byte(0x0f);
        self.write_byte(0xa8);
    }

    /// Near return to calling procedure.
    pub fn ret_near(&mut self) {
        self.write_byte(0xc3); // opcode
    }

    /// Far return to calling procedure.
    pub fn ret_far(&mut self) {
        self.write_byte(0xcb); // opcode
    }

    /// Far return to calling procedure and pop *imm16* bytes from stack.
    pub fn ret_near_imm16(&mut self, imm16: u16) {
        self.write_byte(0xc2);
        self.write_word(imm16);
    }

    /// Far return to calling procedure and pop *imm16* bytes from stack.
    pub fn ret_far_imm16(&mut self, imm16: u16) {
        self.write_byte(0xca);
        self.write_word(imm16);
    }
}