// This is free and unencumbered software released into the public domain.

use clientele::{StandardOptions, SysexitsError, crates::clap::Parser};

/// lsegpu utility from Libegpu <https://libegpu.dev>
#[derive(Debug, Parser)]
#[command(name = "lsegpu")]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// Show all GPU devices, controllers, and enclosures
    #[clap(short = 'a', long, value_parser, global = true)]
    pub all: bool,
}

pub fn main() -> Result<(), SysexitsError> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    #[cfg(feature = "usb")]
    {
        if options.all {
            //println!("# eGPU Enclosures:");
            for enclosure in egpu::list_enclosures().unwrap() {
                println!("- {:#?}", enclosure);
            }
        }
    }

    #[cfg(feature = "pci")]
    {
        if options.all {
            //println!("# eGPU Controllers:");
            for device in egpu::pci::list_controllers().unwrap() {
                println!("- {:#?}", device);
            }
        }

        //println!("# eGPU Devices:");
        for device in egpu::list_devices().unwrap() {
            if device.is_removable().unwrap_or(false) || options.all {
                println!("- {:#?}", device);
            }
        }
    }

    Ok(())
}
