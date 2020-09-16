extern crate libloading as lib;

use std::{
    error::Error,
    result::Result,
};

use plugin_model::ConvertToPlugin;
use plugin_system::scan_all_libraries;

#[cfg(debug_assertions)]
#[cfg(windows)]
const PLUGINS_PATH: &str = ".\\target\\debug";

#[cfg(not(debug_assertions))]
#[cfg(windows)]
const PLUGINS_PATH: &str = ".\\target\\release";

#[cfg(debug_assertions)]
#[cfg(unix)]
const PLUGINS_PATH: &str = "./target/debug";

#[cfg(not(debug_assertions))]
#[cfg(unix)]
const PLUGINS_PATH: &str = "./target/release";

fn main() -> Result<(), Box<dyn Error>> {
    let libraries = scan_all_libraries(PLUGINS_PATH)?;

    for library in libraries {
        let plugin = library?;
        let demo_plugin = plugin.library.to_demo_plugin();

        match demo_plugin {
            Ok(demo_plugin) => println!("{}[{:?}]: {:?}", plugin.name, plugin.path, demo_plugin.get_manifest()),
            Err(err) => println!("{}: did not find a demo plugin!\n{:?}", plugin.name, err),
        }
    }

    Ok(())
}
