use core::ffi::c_char;

use limine::request::RsdpRequest;

use crate::bit_manipulation::any_as_u8_slice;

static RSDP: RsdpRequest = RsdpRequest::new();

#[derive(Debug)]
pub enum RSDPError {
    NoResponse,
    InvalidChecksum
}

pub fn get_rsdp() -> Result<RSDP<'static>, RSDPError>{
    if let Some(rsdp) = RSDP.get_response() {
        let addr = rsdp.address();
        let rsdp = RSDP::new(addr).ok_or(RSDPError::InvalidChecksum)?;
        // log::info!("Found RSDP at {}", rsdp.addr());
        Ok(rsdp)
    } else {
        log::error!("Failed getting rsdp response!");
        Err(RSDPError::NoResponse)
    }
}


#[derive(Debug, Clone)]
pub enum RSDP<'a> {
    RSDP(&'a RawRSDP),
    XSDP(&'a RawXSDP),
}
impl RSDP<'_> {
    pub fn new<'a>(addr: *const ()) -> Option<Self> {
        let _self = unsafe {&*(addr as *const RawRSDP)};
        if _self.revision == 2 {
            let _self = unsafe {&*(addr as *const RawXSDP)};
            if Self::validate_chcksum_xsdp(_self) == false {
                log::error!("Failed validating xsdt checksum !");
                return None
            }
            return Some(RSDP::XSDP(_self))
        }
        if Self::validate_chcksum_rsdp(_self) == false {
            log::error!("Failed validating checksum !");
            return None
        }
        Some(RSDP::RSDP(_self))
    }
    pub fn addr(&self) -> u64 {
        match self {
            RSDP::RSDP(r) => r.rsdt_addr as u64,
            RSDP::XSDP(x) => x.xsdt_addr,
        }
    }
    /// Returns true if checksum is valid
    fn validate_chcksum_rsdp(rsdp: &RawRSDP) -> bool {
        let mut sum: u8 = 0;// Don't use iter.sum cuz we want overflowing adds
        for b in any_as_u8_slice(rsdp) {
            sum = sum.overflowing_add(*b).0;
        }
        sum == 0
    }
    /// Returns true if checksum is valid
    fn validate_chcksum_xsdp(xsdp: &RawXSDP) -> bool {
        let mut sum: u8 = 0;// Don't use iter.sum cuz we want overflowing adds
        for b in any_as_u8_slice(xsdp) {
            sum = sum.overflowing_add(*b).0;
        }
        sum == 0
    }
}
#[repr(packed)]
#[derive(Debug, Clone)]
pub struct RawRSDP {
    sig: [c_char; 8],
    chcksum: u8,
    oem_id: [c_char; 6],
    revision: u8,
    pub rsdt_addr: u32,
}
#[repr(packed)]
#[derive(Debug, Clone)]
pub struct RawXSDP {
    sig: [c_char; 8],
    chcksum: u8,
    oem_id: [c_char; 6],
    revision: u8,
    _rsdt_addr: u32, // Deprecated

    pub len: u32,
    pub xsdt_addr: u64,
    extended_chcksum: u8,
    reserved: [u8; 3],
}