use plugin_model::PluginManifest;

use crate::get_manifest;

#[test]
fn plugin_manifest() {
    let actual = get_manifest();
    let expected = PluginManifest {
        name: "plugin-demo",
        author: "me@me.me",
        meta_data: "Does Things",
    };

    assert_eq!(actual, expected);
}
