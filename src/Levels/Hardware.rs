use ::bevy::prelude::*;



pub mod Processor;
pub mod Ethernet;
pub mod Disk;



pub struct HardwarePlugin;
impl Plugin for HardwarePlugin
{
    fn build(&self, app: &mut bevy::prelude::App)
    {
        if !app.is_plugin_added::<Processor::ProcessorPlugin>() {
            app.add_plugins(Processor::ProcessorPlugin);
        }
        if !app.is_plugin_added::<Ethernet::EthernetPlugin>() {
            app.add_plugins(Ethernet::EthernetPlugin);
        }
        if !app.is_plugin_added::<Disk::DiskPlugin>() {
            app.add_plugins(Disk::DiskPlugin);
        }
    }
}
