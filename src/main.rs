use krajc::prelude::*;
use libloading::Library;
use stabby::libloading::StabbyLibrary;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let game = args.get(1).expect("didnt provide a game to run").clone();

    let game_path = get_game_plugin_path(game);

    let lib = unsafe { Library::new(game_path).unwrap() };
    let plugin = unsafe {
        StabbyLibrary::get_stabbied::<extern "C" fn() -> SystemPlugin>(&lib, b"get_plugin")
    }
    .unwrap()();

    (plugin.register_systems)(SystemPluginRegister::new());

    run(game).await;
}
