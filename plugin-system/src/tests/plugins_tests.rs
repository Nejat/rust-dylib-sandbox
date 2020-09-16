extern crate libloading as lib;

#[cfg(unix)]
use lib::os::unix::*;
#[cfg(windows)]
use lib::os::windows::*;

use plugin_model::PluginManifest;

use crate::PlugIn;
use crate::plugins::{scan_all_libraries, scan_libraries};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type ManifestGetter = extern fn() -> PluginManifest;

#[cfg(debug_assertions)]
#[cfg(windows)]
const PLUGINS_PATH: &str = "..\\target\\debug";

#[cfg(not(debug_assertions))]
#[cfg(windows)]
const PLUGINS_PATH: &str = "..\\target\\release";

#[cfg(debug_assertions)]
#[cfg(unix)]
const PLUGINS_PATH: &str = "../target/debug";

#[cfg(not(debug_assertions))]
#[cfg(unix)]
const PLUGINS_PATH: &str = "../target/release";

#[test]
fn given_dynamic_libraries_in_a_folder_scan_libraries_should_find_libraries() {
    match scan_libraries(PLUGINS_PATH) {
        Ok(plugins) => {
            for plugin in plugins.filter(plugin_demo_only) {
                match plugin {
                    Ok(plugin) => {
                        let get_manifest: Option<Symbol<ManifestGetter>> = unsafe {
                            match plugin.library.get(b"get_manifest") {
                                Ok(func) => Some(func),
                                Err(_) => None,
                            }
                        };

                        if let Some(get_manifest) = get_manifest {
                            let actual = format!("{}: {:?}", plugin.name, get_manifest());
                            let expected = format!("plugin_demo: PluginManifest {{ name: \"plugin-demo\", author: \"me@me.me\", meta_data: \"Does Things\" }}");

                            assert_eq!(actual, expected);
                        } else {
                            panic!(format!("{}: expected get_manifest function!", plugin.name));
                        }
                    }
                    Err(err) => panic!(format!("{}", err)),
                }
            }
        }
        Err(err) => panic!(format!("{}", err)),
    }
}

#[test]
fn given_dynamic_libraries_in_folders_scan_all_libraries_should_find_all_libraries() {
    let mut count = 0;

    match scan_all_libraries(PLUGINS_PATH) {
        Ok(plugins) => {
            for plugin in plugins.filter(plugin_demo_only) {
                match plugin {
                    Ok(plugin) => {
                        let get_manifest: Option<Symbol<ManifestGetter>> = unsafe {
                            match plugin.library.get(b"get_manifest") {
                                Ok(func) => Some(func),
                                Err(_) => None,
                            }
                        };

                        if let Some(get_manifest) = get_manifest {
                            count += 1;

                            let actual = format!("{}: {:?}", plugin.name, get_manifest());
                            let expected = format!("plugin_demo: PluginManifest {{ name: \"plugin-demo\", author: \"me@me.me\", meta_data: \"Does Things\" }}");

                            assert_eq!(actual, expected);
                        } else {
                            panic!(format!("{}: expected get_manifest function!", plugin.name));
                        }
                    }
                    Err(err) => panic!(format!("{}", err)),
                }
            }
        }
        Err(err) => panic!(format!("{}", err)),
    }

    assert!(count > 1)
}

fn plugin_demo_only(plugin: &Result<PlugIn>) -> bool {
    match plugin {
        Ok(plugin) => plugin.name == "plugin_demo",
        Err(_) => false,
    }
}