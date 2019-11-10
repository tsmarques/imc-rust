pub mod Header;
pub mod Message;
pub mod Heartbeat;
pub mod LoggingControl;

// IMC Synchronization number
pub const DUNE_IMC_CONST_SYNC :u16 = 0xFE54;

// IMC's header size
pub const IMC_CONST_HEADER_SIZE :u8 = 20;

// IMC's footer size
pub const IMC_CONST_FOOTER_SIZE :u8 = 2;

// Max size a log's path/name
pub const IMC_MAX_LOG_NAME_SIZE :u16 = 16;

// Unknown entity identifier.
pub const IMC_CONST_UNK_EID :u8 = 255;
