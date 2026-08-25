use ant_war::AntWarPlugin;
use bevy::prelude::*;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(AntWarPlugin);
    app.run()
}
