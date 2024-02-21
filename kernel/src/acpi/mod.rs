use spin::RwLock;

use self::sdt::tables::TablesManager;

pub mod rsdp;
pub mod sdt;

static mut TABLES_MANAGER: Option<RwLock<TablesManager>> = None;

pub fn init() {
    let rsdp = rsdp::get_rsdp().unwrap();
    log::info!("Getting RSDT...");
    //TODO Page mapping
    return;
    let rsdt = sdt::rsdt::get_rsdt(rsdp).unwrap();
    let tables_manager = sdt::tables::parse_rsdt(rsdt).unwrap();
}
