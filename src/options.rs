use std::cmp::min;
use std::fs::read_dir;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

use crate::error::*;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "D. Scott Boggs")]
pub struct Options {
  #[clap(long, default_value = "0")]
  pub level: i32,
  #[clap(short, long, default_value = "0")]
  pub increase_by: i32,
  #[clap(short, long, default_value = "0")]
  pub decrease_by: i32,
  #[clap(long, default_value = "error if more than one display present")]
  pub device: String,
  #[clap(short, long)]
  pub list_devices: bool,
}

impl Options {
  fn path(self: &Self, file: &str) -> String {
    format!("{}/intel_backlight/{}", self.device, file)
  }
  fn set_brightness(self: &Self, value: i32) -> io::Result<()> {
    println!("setting brightness to {}.", value);
    let mut file = File::with_options()
      .write(true)
      .open(self.path("brightness"))?;
    write!(file, "{}", value)?;
    Ok(())
  }
  fn read_int_from_file(path: &str) -> Result<i32> {
    let mut file = File::open(path)?;
    let mut buf: Vec<u8> = Vec::new();
    let size = file.read_to_end(&mut buf)?;
    let string = String::from_utf8(buf)?;
    let string = string.trim_end();
    println!("{}", string);
    if size > 0 {
      Ok(string.parse::<i32>()?)
    } else {
      Err(Error::Generic(format!("file {} was empty", path)))
    }
  }
  fn max_brightness(self: &Self) -> Result<i32> {
    let path = &self.path("max_brightness");
    Self::read_int_from_file(path)
  }
  fn current_brightness(self: &Self) -> Result<i32> {
    let path = &self.path("brightness");
    Self::read_int_from_file(path)
  }

  pub fn update(self: &Self) {
    match self.max_brightness() {
      Ok(max) => {
        if self.level != 0 {
          if let Err(err) = self.set_brightness(self.level) {
            Error::from(err).fatal(ExitCode::FailedToSetBrightnesss);
          }
        } else {
          match self.current_brightness() {
            Ok(current) => {
              if let Err(err) =
                self.set_brightness(min(max, current + self.increase_by - self.decrease_by))
              {
                Error::from(err).fatal(ExitCode::FailedToSetBrightnesss)
              }
            }
            Err(err) => Error::from(err).fatal(ExitCode::FailedToReadCurrentBrightness),
          }
        }
      }
      Err(err) => Error::from(err).fatal(ExitCode::FailedToReadMaxBrightness),
    }
  }
  pub fn get_devices_list(
    self: &Self,
  ) -> Result<
    std::iter::Filter<
      std::fs::ReadDir,
      fn(&std::result::Result<std::fs::DirEntry, std::io::Error>) -> bool,
    >,
  > {
    if let Ok(dir_iter) = read_dir("/sys/class/drm") {
      Ok(dir_iter.filter(|dev| {
        if let Ok(dev) = dev {
          dev
            .path()
            .join("intel_backlight")
            .join("brightness")
            .exists()
        } else {
          false
        }
      }))
    } else {
      Err(Error::from("error finding devices"))
    }
  }
  pub fn do_list_devices(self: &Self) {
    if self.list_devices {
      match self.get_devices_list() {
        Ok(devices) => {
          for device in devices {
            if let Ok(device) = device {
              if let Some(name) = device.path().file_name() {
                if let Some(text) = name.to_str() {
                  println!("{}", text);
                } else {
                  println!("{}", device.path().display())
                }
              } else {
                println!("{}", device.path().display())
              }
            }
          }
          exit(ExitCode::Ok as i32);
        }
        Err(err) => err.fatal(ExitCode::FailedToFindDevices),
      }
    }
  }
}
