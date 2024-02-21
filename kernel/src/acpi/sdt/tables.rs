use super::{apic::APIC, fadt::FADT, rsdt::RSDT};

/// https://wiki.osdev.org/RSDT#What_can_you_find.3F
#[derive(Debug, Default)]
pub struct TablesManager {
    fadt: Option<&'static FADT>,
    apic: Option<&'static APIC>,
}

pub fn parse_rsdt(rsdt: RSDT<'static>) -> Option<TablesManager> {
    log::info!("Parsing RSDT...");
    Some(TablesManager {
        fadt: todo!(),
        apic: todo!(),
    })
}
#[derive(Debug, Clone, Copy)]
enum Signature {
    APIC,
    BERT,
    CPEP,
    DSDT,
    ECDT,
    EINJ,
    ERST,
    FACP,
    FACS,
    HEST,
    MSCT,
    MPST,
    OEMx,
    PMTT,
    PSDT,
    RASF,
    RSDT,
    SBST,
    SLIT,
    SRAT,
    SSDT,
    XSDT,
}
const TABLES_SIGNATURES: &[(&str, Signature)] = &[
    ("APIC", Signature::APIC), // Multiple APIC Description Table (MADT)
    ("BERT", Signature::BERT), // Boot Error Record Table (BERT)
    ("CPEP", Signature::CPEP), // Corrected Platform Error Polling Table (CPEP)
    ("DSDT", Signature::DSDT), // Differentiated System Description Table (DSDT)
    ("ECDT", Signature::ECDT), // Embedded Controller Boot Resources Table (ECDT)
    ("EINJ", Signature::EINJ), // Error Injection Table (EINJ)
    ("ERST", Signature::ERST), // Error Record Serialization Table (ERST)
    ("FACP", Signature::FACP), // Fixed ACPI Description Table (FADT)
    ("FACS", Signature::FACS), // Firmware ACPI Control Structure (FACS)
    ("HEST", Signature::HEST), // Hardware Error Source Table (HEST)
    ("MSCT", Signature::MSCT), // Maximum System Characteristics Table (MSCT)
    ("MPST", Signature::MPST), // Memory Power State Table (MPST)
    ("OEMx", Signature::OEMx), // OEM Specific Information Tables (Any table with a signature beginning with "OEM" falls into this definition)
    ("PMTT", Signature::PMTT), // latform Memory Topology Table (PMTT)
    ("PSDT", Signature::PSDT), // Persistent System Description Table (PSDT)
    ("RASF", Signature::RASF), // ACPI RAS Feature Table (RASF)
    ("RSDT", Signature::RSDT), // Root System Description Table (This wiki page; included for completeness)
    ("SBST", Signature::SBST), // Smart Battery Specification Table (SBST)
    ("SLIT", Signature::SLIT), // System Locality System Information Table (SLIT)
    ("SRAT", Signature::SRAT), // System Resource Affinity Table (SRAT)
    ("SSDT", Signature::SSDT), // Secondary System Description Table (SSDT)
    ("XSDT", Signature::XSDT), // Extended System Description Table (XSDT; 64-bit version of the RSDT
];