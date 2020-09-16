use libloading::Error;
#[cfg(unix)]
use libloading::os::unix::Library;
#[cfg(windows)]
use libloading::os::windows::Library;

use crate::DemoPlugin;

pub trait ConvertToPlugin {
    fn to_demo_plugin(&self) -> Result<DemoPlugin, Error>;
}

impl ConvertToPlugin for Library {
    fn to_demo_plugin(&self) -> Result<DemoPlugin, Error> {
        Ok(
            DemoPlugin {
                get_manifest_symbol: unsafe {
                    self.get(b"get_manifest")?
                }
            }
        )
    }
}
