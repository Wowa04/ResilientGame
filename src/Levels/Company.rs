use ::bevy::prelude::Plugin;



pub struct CompanyPlugin;
impl Plugin for CompanyPlugin
{
    fn build(&self, app: &mut bevy::prelude::App)
    {
        if !app.is_plugin_added::<Server::ServerPlugin>() {
            app.add_plugins(Server::ServerPlugin);
        }
    }
}



pub mod Server;