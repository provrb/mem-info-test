use serde::Deserialize;
use wmi::{datetime, COMLibrary, WMIConnection};

#[derive(Deserialize, Debug)]
struct PhysicalMemory {
    Attributes: Option<u32>,
    BankLabel: Option<String>,
    Capacity: Option<String>,
    Caption: Option<String>,
    ConfiguredClockSpeed: Option<u32>,
    ConfiguredVoltage: Option<u32>,
    CreationClassName: Option<String>,
    DataWidth: Option<u16>,
    Description: Option<String>,
    DeviceLocator: Option<String>,
    FormFactor: Option<u16>,
    HotSwappable: Option<bool>,
    InstallDate: Option<datetime::WMIDateTime>,
    InterleaveDataDepth: Option<u16>,
    InterleavePosition: Option<u32>,
    Manufacturer: Option<String>,
    MaxVoltage: Option<u32>,
    MemoryType: Option<u16>,
    MinVoltage: Option<u32>,
    Model: Option<String>,
    Name: Option<String>,
    OtherIdentifyingInfo: Option<String>,
    PartNumber: Option<String>,
    PositionInRow: Option<u32>,
    PoweredOn: Option<bool>,
    Removable: Option<bool>,
    Replaceable: Option<bool>,
    SerialNumber: Option<String>,
    SKU: Option<String>,
    SMBIOSMemoryType: Option<u32>,
    Speed: Option<u32>,
    Status: Option<String>,
    Tag: Option<String>,
    TotalWidth: Option<u16>,
    TypeDetail: Option<u16>,
    Version: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize COM library
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    // Query the Win32_PhysicalMemory class
    let results: Vec<PhysicalMemory> = wmi_con.raw_query("SELECT * FROM Win32_PhysicalMemory")?;
    dbg!(results);

    Ok(())
}
