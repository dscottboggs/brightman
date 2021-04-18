#![feature(with_options)]
mod error;
mod options;
use crate::error::*;
use crate::options::Options;
use clap::Clap;

fn main() {
    let mut options = Options::parse();
    options.do_list_devices();
    if let (0, 0, 0) = (options.level, options.increase_by, options.decrease_by) {
        Error::from("you must specify a level to set, an increment amount, or amount to decrease")
            .fatal(ExitCode::ArgumentError)
    };
    if options.device == "" || options.device == "error if more than one display present" {
        match options.get_devices_list() {
            Ok(devices) => {
                let devices: Vec<std::result::Result<std::fs::DirEntry, std::io::Error>> =
                    devices.collect();
                if devices.len() == 1 {
                    if let Ok(device) = &devices[0] {
                        if let Some(path) = device.path().to_str() {
                            options.device = path.to_string();
                        } else {
                            Error::from(format!("failed to load the only device {:?}", device))
                                .fatal(ExitCode::FailedToFindDevices)
                        }
                    } else {
                        Error::from("failed to read device diretory... is this a Linux system?")
                            .fatal(ExitCode::FailedToFindDevices);
                    }
                } else {
                    Error::from("device not specified").fatal(ExitCode::ArgumentError)
                }
            }
            Err(err) => Error::from(err).fatal(ExitCode::FailedToFindDevices),
        }
    }
    options.update();
}
