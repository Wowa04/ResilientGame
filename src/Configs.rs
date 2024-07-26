use ::bevy::app::Plugin;

pub mod World;
pub mod Player;



pub struct ConfigPlugin;
impl Plugin for ConfigPlugin
{
    fn build(&self, app: &mut bevy::prelude::App)
    {
        if !app.is_plugin_added::<World::WorldPlugin>() {
            app.add_plugins(World::WorldPlugin);
        }
        if !app.is_plugin_added::<Player::PlayerPlugin>() {
            app.add_plugins(Player::PlayerPlugin);
        }
    }
}
