//! Registers in x86_64 assembly.

/// An 8-bit register.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg8 {
    Al,
    Bl,
    Cl,
    Dl,
    Sil,
    Dil,
    Bpl,
    Spl,
    Ah,
    Bh,
    Ch,
    Dh,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
}

impl Reg8 {
    /// The offset of this register, for instruction encoding.
    pub fn offset(&self) -> u8 {
        match self {
            Self::Al => 0,
            Self::Cl => 1,
            Self::Dl => 2,
            Self::Bl => 3,
            Self::Spl => 4,
            Self::Bpl => 5,
            Self::Sil => 6,
            Self::Dil => 7,
            Self::Ah => 4,
            Self::Ch => 5,
            Self::Dh => 6,
            Self::Bh => 7,
            Self::R8b => 0,
            Self::R9b => 1,
            Self::R10b => 2,
            Self::R11b => 3,
            Self::R12b => 4,
            Self::R13b => 5,
            Self::R14b => 6,
            Self::R15b => 7,
        }
    }

    /// Returns true if this register is one of the extension registers, such as `r8`
    /// 
    /// # Example
    /// ```
    /// use asmkit_x86_64::register::Reg32;
    /// 
    /// let r8d = Reg32::R8d;
    /// let eax = Reg32::Eax;
    /// 
    /// assert!(r8d.is_extension());
    /// assert!(!eax.is_extension());
    /// ```
    pub fn is_extension(&self) -> bool {
        match self {
            Self::R8b | Self::R9b | Self::R10b | Self::R11b | Self::R12b | Self::R13b | Self::R14b | Self::R15b => true,
            _ => false,
        }
    }

    /// Returns true if this register is reserved for a specific purpose, such as `spl`.
    pub fn is_reserved(&self) -> bool {
        match self {
            Self::Spl | Self::Bpl | Self::Sil | Self::Dil => true,
            _ => false,
        }
    }
}

/// A 16-bit register.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg16 {
    Ax,
    Bx,
    Cx,
    Dx,
    Si,
    Di,
    Bp,
    Sp,
    R8w,
    R9w,
    R10w,
    R11w,
    R12w,
    R13w,
    R14w,
    R15w,
}

impl Reg16 {
    /// The offset of this register, for instruction encoding.
    pub fn offset(&self) -> u8 {
        match self {
            Self::Ax => 0,
            Self::Cx => 1,
            Self::Dx => 2,
            Self::Bx => 3,
            Self::Sp => 4,
            Self::Bp => 5,
            Self::Si => 6,
            Self::Di => 7,
            Self::R8w => 0,
            Self::R9w => 1,
            Self::R10w => 2,
            Self::R11w => 3,
            Self::R12w => 4,
            Self::R13w => 5,
            Self::R14w => 6,
            Self::R15w => 7,
        }
    }

    /// Returns true if this register is one of the extension registers, such as `r8`
    /// 
    /// # Example
    /// ```
    /// use asmkit_x86_64::register::Reg32;
    /// 
    /// let r8d = Reg32::R8d;
    /// let eax = Reg32::Eax;
    /// 
    /// assert!(r8d.is_extension());
    /// assert!(!eax.is_extension());
    /// ```
    pub fn is_extension(&self) -> bool {
        match self {
            Self::R8w | Self::R9w | Self::R10w | Self::R11w | Self::R12w | Self::R13w | Self::R14w | Self::R15w => true,
            _ => false,
        }
    }
}

/// A 32-bit register.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg32 {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Ebp,
    Esp,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,
}

impl Reg32 {
    /// The offset of this register, for instruction encoding.
    pub fn offset(&self) -> u8 {
        match self {
            Self::Eax => 0,
            Self::Ecx => 1,
            Self::Edx => 2,
            Self::Ebx => 3,
            Self::Esp => 4,
            Self::Ebp => 5,
            Self::Esi => 6,
            Self::Edi => 7,
            Self::R8d => 0,
            Self::R9d => 1,
            Self::R10d => 2,
            Self::R11d => 3,
            Self::R12d => 4,
            Self::R13d => 5,
            Self::R14d => 6,
            Self::R15d => 7,
        }
    }

    /// Returns true if this register is one of the extension registers, such as `r8d`
    /// 
    /// # Example
    /// ```
    /// use asmkit_x86_64::register::Reg32;
    /// 
    /// let r8d = Reg32::R8d;
    /// let eax = Reg32::Eax;
    /// 
    /// assert!(r8d.is_extension());
    /// assert!(!eax.is_extension());
    /// ```
    pub fn is_extension(&self) -> bool {
        match self {
            Self::R8d | Self::R9d | Self::R10d | Self::R11d | Self::R12d | Self::R13d | Self::R14d | Self::R15d => true,
            _ => false,
        }
    }
}

/// A 64-bit register.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg64 {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Reg64 {
    /// The offset of this register, for instruction encoding.
    pub fn offset(&self) -> u8 {
        match self {
            Self::Rax => 0,
            Self::Rcx => 1,
            Self::Rdx => 2,
            Self::Rbx => 3,
            Self::Rsp => 4,
            Self::Rbp => 5,
            Self::Rsi => 6,
            Self::Rdi => 7,
            Self::R8 => 0,
            Self::R9 => 1,
            Self::R10 => 2,
            Self::R11 => 3,
            Self::R12 => 4,
            Self::R13 => 5,
            Self::R14 => 6,
            Self::R15 => 7,
        }
    }

    /// Returns true if this register is one of the extension registers, such as `r8`
    /// 
    /// # Example
    /// ```
    /// use asmkit_x86_64::register::Reg64;
    /// 
    /// let r8 = Reg64::R8;
    /// let rax = Reg64::Rax;
    /// 
    /// assert!(r8.is_extension());
    /// assert!(!rax.is_extension());
    /// ```
    pub fn is_extension(&self) -> bool {
        match self {
            Self::R8 | Self::R9 | Self::R10 | Self::R11 | Self::R12 | Self::R13 | Self::R14 | Self::R15 => true,
            _ => false,
        }
    }
}
