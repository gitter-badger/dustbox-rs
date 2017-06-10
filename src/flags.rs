// TODO later: look into bitflags! macro

// https://en.wikipedia.org/wiki/FLAGS_register
pub struct Flags {
    pub carry: bool, // 0: carry flag
    reserved1: bool, // 1: Reserved, always 1 in EFLAGS
    pub parity: bool, // 2: parity flag
    reserved3: bool,
    pub auxiliary_carry: bool, // 4: auxiliary carry flag
    reserved5: bool,
    pub zero: bool, // 6: zero flag
    pub sign: bool, // 7: sign flag
    pub trap: bool, // 8: trap flag (single step)
    pub interrupt: bool, // 9: interrupt flag
    pub direction: bool, // 10: direction flag (control with cld, std)
    pub overflow: bool, // 11: overflow flag
    iopl12: bool, // 12: I/O privilege level (286+ only), always 1 on 8086 and 186
    iopl13: bool, // 13 --""---
    nested_task: bool, // 14: Nested task flag (286+ only), always 1 on 8086 and 186
    reserved15: bool, // 15: Reserved, always 1 on 8086 and 186, always 0 on later models
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            carry: false,
            reserved1: false,
            parity: false,
            reserved3: false,
            auxiliary_carry: false,
            reserved5: false,
            zero: false,
            sign: false,
            trap: false,
            interrupt: false,
            direction: false,
            overflow: false,
            iopl12: false,
            iopl13: false,
            nested_task: false,
            reserved15: false,
        }
    }
    pub fn set_sign_u8(&mut self, v: usize) {
        // Set equal to the most-significant bit of the result,
        // which is the sign bit of a signed integer.
        // (0 indicates a positive value and 1 indicates a negative value.)
        self.sign = v & 0x80 != 0;
    }
    pub fn set_sign_u16(&mut self, v: usize) {
        self.sign = v & 0x8000 != 0;
    }
    pub fn set_parity(&mut self, v: usize) {
        // Set if the least-significant byte of the result contains an
        // even number of 1 bits; cleared otherwise.
        self.parity = v & 1 == 0;
    }
    pub fn set_zero_u8(&mut self, v: usize) {
        // Zero flag — Set if the result is zero; cleared otherwise.
        self.zero = (v & 0xFF) == 0;
    }
    pub fn set_zero_u16(&mut self, v: usize) {
        self.zero = (v & 0xFFFF) == 0;
    }
    pub fn set_auxiliary(&mut self, res: usize, v1: usize, v2: usize) {
        // Set if an arithmetic operation generates a carry or a borrow out
        // of bit 3 of the result; cleared otherwise. This flag is used in
        // binary-coded decimal (BCD) arithmetic.
        self.auxiliary_carry = (res ^ (v1 ^ v2)) & 0x10 != 0;
    }
    pub fn set_overflow_add_u8(&mut self, res: usize, v1: usize, v2: usize) {
        // Set if the integer result is too large a positive number or too
        // small a negative number (excluding the sign-bit) to fit in the
        // destination operand; cleared otherwise. This flag indicates an
        // overflow condition for signed-integer (two’s complement) arithmetic.
        self.overflow = (res ^ v1) & (res ^ v2) & 0x80 != 0;
    }
    pub fn set_overflow_add_u16(&mut self, res: usize, v1: usize, v2: usize) {
        self.overflow = (res ^ v1) & (res ^ v2) & 0x8000 != 0;
    }
    pub fn set_overflow_sub_u8(&mut self, res: usize, v1: usize, v2: usize) {
        self.overflow = (v2 ^ v1) & (v2 ^ res) & 0x80 != 0;
    }
    pub fn set_overflow_sub_u16(&mut self, res: usize, v1: usize, v2: usize) {
        self.overflow = (v2 ^ v1) & (v2 ^ res) & 0x8000 != 0;
    }
    pub fn set_carry_u8(&mut self, res: usize) {
        // Set if an arithmetic operation generates a carry or a borrow out of
        // the most-significant bit of the result; cleared otherwise. This flag
        // indicates an overflow condition for unsigned-integer arithmetic.
        self.carry = res & 0x100 != 0;
    }
    pub fn set_carry_u16(&mut self, res: usize) {
        self.carry = res & 0x10000 != 0;
    }
    pub fn set_u16(&mut self, val: u16) {
        println!("XXX impl flags.set_u16 = {:04X}", val);
    }

    // returns the FLAGS register
    pub fn u16(&self) -> u16 {
        let mut val = 0 as u16;

        if self.carry {
            val |= 1;
        }
        if self.parity {
            val |= 1 << 2;
        }
        if self.auxiliary_carry {
            val |= 1 << 4;
        }
        if self.zero {
            val |= 1 << 6;
        }
        if self.sign {
            val |= 1 << 7;
        }
        if self.trap {
            val |= 1 << 8;
        }
        if self.interrupt {
            val |= 1 << 9;
        }
        if self.direction {
            val |= 1 << 10;
        }
        if self.overflow {
            val |= 1 << 11;
        }
        if self.iopl12 {
            val |= 1 << 12;
        }
        if self.iopl13 {
            val |= 1 << 13;
        }
        if self.nested_task {
            val |= 1 << 14;
        }
        val |= 1 << 15; // always 1 on 8086 and 186, always 0 on later models

        val
    }
}

