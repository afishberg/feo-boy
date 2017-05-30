//! Error handling.

use std::io;

error_chain! {
    foreign_links {
        Io(io::Error);
    }

    errors {
        InvalidBios(reason: String) {
            description("invalid BIOS")
            display("invalid BIOS: {}", reason)
        }
        InvalidCartridge(reason: String) {
            description("invalid cartridge ROM")
            display("invalid cartridge ROM: {}", reason)
        }
    }
}
