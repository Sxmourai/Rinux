use crate::acpi::rsdp::RSDP;

use super::ACPISDTHeader;

#[derive(Debug, Clone)]
pub enum RSDTError {
    InvalidChecksum
}
pub enum RSDT<'a> {
    RSDT(&'a RawRSDT),
    XSDT(&'a RawXSDT),
}
pub fn get_rsdt(rsdp: RSDP) -> Result<RSDT<'static>, RSDTError> {
    match rsdp {
        RSDP::RSDP(rsdp) => {
            let raw_rsdt = unsafe {&*(rsdp.rsdt_addr as *const RawRSDT)};
            if raw_rsdt.sdt.validate_chcksum() ==false {return Err(RSDTError::InvalidChecksum)}
            log::info!("Validated RSDT");
        
            Ok(RSDT::RSDT(raw_rsdt))
        },
        RSDP::XSDP(xsdp) => {
            let raw_xsdt = unsafe {&*(xsdp.xsdt_addr as *const RawXSDT)};
            if raw_xsdt.sdt.validate_chcksum() ==false {return Err(RSDTError::InvalidChecksum)}
            log::info!("Validated RSDT");
        
            Ok(RSDT::XSDT(raw_xsdt))
        },
    }
}


#[repr(packed)]
#[derive(Debug, Clone)]
pub struct RawRSDT {
    sdt: ACPISDTHeader,
}
impl RawRSDT {
    pub fn other_sdt<'a>(&'a self) -> &'a [u32] {
        let len = (self.sdt.len as usize-core::mem::size_of::<ACPISDTHeader>())/4;
        unsafe {core::slice::from_raw_parts((core::ptr::addr_of!(self) as *const u32).add(core::mem::size_of::<ACPISDTHeader>()), len)}
    }
}
#[repr(packed)]
#[derive(Debug, Clone)]
pub struct RawXSDT {
    sdt: ACPISDTHeader,
}
impl RawXSDT {
    pub fn other_sdt<'a>(&'a self) -> &'a [u64] {
        let len = (self.sdt.len as usize-core::mem::size_of::<ACPISDTHeader>())/8;
        unsafe {core::slice::from_raw_parts((core::ptr::addr_of!(self) as *const u64).add(core::mem::size_of::<ACPISDTHeader>()), len)}
    }
}