use memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct MMU {
    pub memory: Rc<RefCell<Memory>>
}

impl MMU {
    pub fn new() -> Self{
        MMU {
            memory: Rc::new(RefCell::new(Memory::new()))
        }
    }

    pub fn s_translate(&self, seg: u16, offset: u16) -> usize {
        let seg = seg as usize;
        let offset = offset as usize;

        seg * 16 + offset
    }

    pub fn read_u8(&self, seg: u16, offset: u16) -> u8 {
        self.memory.borrow().read_u8(self.s_translate(seg, offset))
    }

    pub fn read_u16(&self, seg: u16, offset: u16) -> u16 {
        self.memory.borrow().read_u16(self.s_translate(seg, offset))
    }

    pub fn write_u8(&mut self, seg: u16, offset: u16, data: u8) {
        let addr = self.s_translate(seg, offset);
        self.memory.borrow_mut().write_u8(
            addr,
            data);
    }

    pub fn write_u16(&mut self, seg: u16, offset: u16, data: u16) {
        let addr = self.s_translate(seg, offset);
        self.memory.borrow_mut().write_u16(
            addr,
            data);
    }

    pub fn read(&self, seg: u16, offset: u16, length: usize) -> Vec<u8> {
        let addr = self.s_translate(seg, offset);
        Vec::from(self.memory.borrow().read(addr, length))
    }

    pub fn write(&mut self, seg: u16, offset: u16, data: &[u8]) {
        let addr = self.s_translate(seg, offset);
        self.memory.borrow_mut().write(addr, data);
    }

    pub fn dump_mem(&self) -> Vec<u8> {
        self.memory.borrow().memory.clone()
    }
}
