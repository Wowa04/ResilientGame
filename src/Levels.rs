use ::bevy::prelude::Plugin;



pub struct LevelsPlugin;
impl Plugin for LevelsPlugin
{
    fn build(&self, app: &mut bevy::prelude::App)
    {
        if !app.is_plugin_added::<Software::SoftwarePlugin>() {
            app.add_plugins(Software::SoftwarePlugin);
        }
        if !app.is_plugin_added::<Hardware::HardwarePlugin>() {
            app.add_plugins(Hardware::HardwarePlugin);
        }
        if !app.is_plugin_added::<Company::CompanyPlugin>() {
            app.add_plugins(Company::CompanyPlugin);
        }
    }
}



pub mod Software;
pub mod Hardware;
pub mod Company;