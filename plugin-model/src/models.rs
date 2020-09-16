#[no_mangle]
#[derive(Debug, Eq, PartialEq)]
pub struct PluginManifest {
    pub name: &'static str,
    pub author: &'static str,
    pub meta_data: &'static str,
}
