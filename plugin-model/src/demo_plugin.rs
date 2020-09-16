#[cfg(unix)]
use libloading::os::unix::Symbol;
#[cfg(windows)]
use libloading::os::windows::Symbol;

use crate::PluginManifest;

type ManifestGetter = extern fn() -> PluginManifest;

pub struct DemoPlugin {
    pub(crate) get_manifest_symbol: Symbol<ManifestGetter>
}

impl DemoPlugin {
    pub fn get_manifest(&self) -> PluginManifest {
        (&*self.get_manifest_symbol)()
    }
}
