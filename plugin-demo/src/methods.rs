use plugin_model::PluginManifest;

#[no_mangle]
pub const fn get_manifest() -> PluginManifest {
    PluginManifest {
        name: "plugin-demo",
        author: "me@me.me",
        meta_data: "Does Things",
    }
}