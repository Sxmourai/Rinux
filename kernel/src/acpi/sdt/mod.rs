pub mod tables;
pub mod rsdt;
pub mod fadt;
pub mod apic;


use core::ffi::c_char;
#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct ACPISDTHeader {
    sig: [c_char; 4],
    len: u32,
    revision: u8,
    chcksum: u8,
    oem_id: [c_char; 6],
    oem_table_id: [c_char; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}
impl ACPISDTHeader {
    pub fn validate_chcksum(&self) -> bool {
        let mut sum: u8 = 0;// Don't use iter.sum cuz we want overflowing adds
        for b in crate::bit_manipulation::any_as_u8_slice(self) {
            sum = sum.overflowing_add(*b).0;
        }
        sum == 0
    }
}