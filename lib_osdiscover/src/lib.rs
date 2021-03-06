extern crate regex;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate env_logger;

mod error;

use std::collections::HashMap;
use std::process::Command;

use regex::Regex;
use failure::Error;

use error::OSDetectionError;

/// Enum with the various types of operating systems
pub enum OS {
    Linux{
        distribution: Option<String>,
        release: Option<String>
    },
    Macos{
        product_name: Option<String>,
        version: Option<String>,
        build_version: Option<String>
    },
    Windows,
    Unknown
}

/// Contains the information collected about the OS
pub struct OSDetector {
    os: Option<OS>
}

impl OSDetector {
    /// Creates and returns a new OSDetector
    pub fn new() -> OSDetector {
        OSDetector{
            os: None
        }
    }

    /// Main function that runs through a bunch of checks to try to figure out what version of what OS the binary is running on
    pub fn detect_os(&self) -> Result<OS, Error> {
        if OSDetector::has_lsb_release() {
            let mut os_info = OS::Linux{distribution: None, release: None};
            if let Ok(lsb_info) = OSDetector::parse_lsb_info() {
                if let OS::Linux{ref mut distribution, ref mut release} = os_info {
                    if lsb_info.contains_key("distribution".into()) {
                        *distribution = lsb_info.get("distribution").unwrap().clone();
                    }

                    if lsb_info.contains_key("release".into()) {
                        *release = lsb_info.get("release").unwrap().clone();
                    }
                }
            }
            return Ok(os_info);
        }

        if OSDetector::has_sw_vers() {
            let mut os_info = OS::Macos{product_name: None, version: None, build_version: None};
            if let Ok(sw_info) = OSDetector::parse_sw_vers() {
                if let OS::Macos{ref mut product_name, ref mut build_version, ref mut version} = os_info {
                    if sw_info.contains_key("product_name".into()) {
                        *product_name = sw_info.get("product_name").unwrap().clone();
                    }

                    if sw_info.contains_key("version".into()) {
                        *version = sw_info.get("version").unwrap().clone();
                    }

                    if sw_info.contains_key("build_version".into()) {
                        *build_version = sw_info.get("build_version").unwrap().clone();
                    }
                }
            }
            return Ok(os_info);
        }

        Ok(OS::Unknown)
    }

    // Checks if the `lsb_release` command is available
    fn has_lsb_release() -> bool {
        if let Ok(output) = Command::new("lsb_release").output() {
            true
        } else {
            false
        }
    }

    // Checks if the `lsb_release` command is available
    fn has_sw_vers() -> bool {
        if let Ok(output) = Command::new("sw_vers").output() {
            true
        } else {
            false
        }
    }

    // Executes `lsb_release -a` and attempts to gather info from it
    fn parse_lsb_info() -> Result<HashMap<String, Option<String>>, Error> {
        let mut results = HashMap::new();
        if let Ok(output) = Command::new("lsb_release").arg("-a").output() {
            let output = String::from_utf8_lossy(&output.stdout);
            let distrib_regex = Regex::new(r"Distributor ID:\s*(\w+)")?;
            let distrib_release_regex = Regex::new(r"Release:\s*([\w\.]+)")?;

            let mut distribution: Option<String> = None;
            let mut release: Option<String> = None;

            if let Some(distrib) = distrib_regex.captures_iter(&output).next() {
                if let Some(d) = distrib.get(1) {
                    distribution = Some(d.as_str().to_owned());
                }
            }

            if let Some(rel) = distrib_release_regex.captures_iter(&output).next() {
                if let Some(r) = rel.get(1) {
                    release = Some(r.as_str().to_owned());
                }
            }

            results.insert("distribution".into(), distribution);
            results.insert("release".into(), release);
            Ok(results)

        } else {
            Err(Error::from(OSDetectionError::LSBReleaseCommandFail))
        }
    }

    // Attempts to use the `sw_ver` command to parse out the version of OSX
    fn parse_sw_vers() -> Result<HashMap<String, Option<String>>, Error> {
        let mut results = HashMap::new();
        if let Ok(output) = Command::new("sw_vers").output() {
            let output = String::from_utf8_lossy(&output.stdout);
            let product_name_regex = Regex::new(r"ProductName:\s*(\w+)")?;
            let product_version_regex = Regex::new(r"ProductVersion:\s*(\w+)")?;
            let build_version_regex = Regex::new(r"BuildVersion:\s*(\w+)")?;

            let mut product_name: Option<String> = None;
            let mut version: Option<String> = None;
            let mut build_version: Option<String> = None;

            if let Some(product) = product_name_regex.captures_iter(&output).next() {
                if let Some(p) = product.get(1) {
                    product_name = Some(p.as_str().to_owned());
                }
            }

            if let Some(build_version) = product_version_regex.captures_iter(&output).next() {
                if let Some(v) = build_version.get(1) {
                    version = Some(v.as_str().to_owned());
                }
            }

            if let Some(version) = build_version_regex.captures_iter(&output).next() {
                if let Some(v) = version.get(1) {
                    build_version = Some(v.as_str().to_owned());
                }
            }

            results.insert("product_name".into(), product_name);
            results.insert("version".into(), version);
            results.insert("build_version".into(), build_version);
            Ok(results)

        } else {
            Err(Error::from(OSDetectionError::SwVersCommandFailed))
        }
    }
}
