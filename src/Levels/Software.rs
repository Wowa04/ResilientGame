use ::bevy::prelude::*;



pub struct SoftwarePlugin;
impl Plugin for SoftwarePlugin
{
    fn build(&self, app: &mut bevy::prelude::App)
    {
        if !app.is_plugin_added::<Packet::PacketPlugin>() {
            app.add_plugins(Packet::PacketPlugin);
        }
        if !app.is_plugin_added::<App::AppPlugin>() {
            app.add_plugins(App::AppPlugin);
        }
        if !app.is_plugin_added::<Internet::InternetPlugin>() {
            app.add_plugins(Internet::InternetPlugin);
        }
    }
}



pub mod Packet;
pub mod App;
pub mod Internet;
