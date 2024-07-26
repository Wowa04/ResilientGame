use ::bevy::prelude::*;
use ::bevy_egui::{egui, EguiContexts};



pub struct WorldPlugin;
impl Plugin for WorldPlugin
{
    fn build(&self, app: &mut bevy::prelude::App)
    {
        app.init_resource::<WorldConfig>();
        app.add_systems(Update, world_ui);
    }
}


#[derive(Resource)]
pub struct WorldConfig
{
    pub simulation                  : bool,
    pub money_per_request           : usize,
    pub antivirus_time              : f32,
    pub antivirus_cost              : usize,
    pub firewall_time               : f32,
    pub firewall_cost               : usize,
    pub disk_failtime               : f32,
    pub disk_packets                : f32,
    pub disk_cost                   : usize,
    pub packet_arrival_delay_user   : f32,
    pub packet_arrival_delay_malware: f32,
    pub packet_arrival_delay_ddos   : f32,
    pub packet_speed                : f32,
}
impl Default for WorldConfig
{
    fn default() -> Self {
        Self {
            simulation                  : false,
            money_per_request           :  5 ,
            antivirus_time              : 30.,
            antivirus_cost              :100 ,
            firewall_time               : 30.,
            firewall_cost               :100 ,
            disk_failtime               : 60.,
            disk_packets                : 50.,
            disk_cost                   :100 ,
            packet_arrival_delay_user   :  9.,
            packet_arrival_delay_malware:  1.,
            packet_arrival_delay_ddos   :  1.,
            packet_speed                : 10.,
        }
    }
}
impl WorldConfig
{
    pub fn get_money_per_request           (&self) -> usize {self.money_per_request}
    pub fn get_antivirus_time              (&self) -> f32   {self.antivirus_time}
    pub fn get_antivirus_cost              (&self) -> usize {self.antivirus_cost}
    pub fn get_firewall_time               (&self) -> f32   {self.firewall_time}
    pub fn get_firewall_cost               (&self) -> usize {self.firewall_cost}
    pub fn get_disk_failtime               (&self) -> f32   {self.disk_failtime}
    pub fn get_disk_packets                (&self) -> f32   {self.disk_packets}
    pub fn get_disk_cost                   (&self) -> usize {self.disk_cost}
    pub fn get_packet_arrival_delay_user   (&self) -> f32   {1. / self.packet_arrival_delay_user   .sqrt()}
    pub fn get_packet_arrival_delay_malware(&self) -> f32   {1. / self.packet_arrival_delay_malware.sqrt()}
    pub fn get_packet_arrival_delay_ddos   (&self) -> f32   {1. / self.packet_arrival_delay_ddos   .sqrt()}
    pub fn get_packet_speed                (&self) -> f32   {self.packet_speed * self.packet_speed}
}



pub fn world_ui(
    // mut commands: Commands,
    mut contexts: EguiContexts,
    mut config  : ResMut<WorldConfig>,
){
    egui::Window::new("World Control").show(
        contexts.ctx_mut(),
        |ui|
        {
            ui.checkbox(&mut config.simulation, "Enable Simulation");
            ui.add(egui::Slider::new(&mut config.money_per_request           , 0    ..= 100 ).text("Money per Request"));
            ui.add(egui::Slider::new(&mut config.antivirus_time              , 1.   ..= 120.).text("Antivirus Time"));
            ui.add(egui::Slider::new(&mut config.antivirus_cost              , 0    ..=1000 ).text("Antivirus Cost"));
            ui.add(egui::Slider::new(&mut config.firewall_time               , 1.   ..= 120.).text("Firewall Time"));
            ui.add(egui::Slider::new(&mut config.firewall_cost               , 0    ..=1000 ).text("Firewall Cost"));
            ui.add(egui::Slider::new(&mut config.disk_failtime               , 0.   ..= 300.).text("Disk Fail Time"));
            ui.add(egui::Slider::new(&mut config.disk_packets                , 1.   ..= 300.).text("Packets per Disk"));
            ui.add(egui::Slider::new(&mut config.disk_cost                   , 0    ..=1000 ).text("Disk Cost"));
            ui.add(egui::Slider::new(&mut config.packet_arrival_delay_user   , 0.01 ..=  20.).text("Packet Arrival Speed (User)"));
            ui.add(egui::Slider::new(&mut config.packet_arrival_delay_malware, 0.01 ..=  20.).text("Packet Arrival Speed (Malware)"));
            ui.add(egui::Slider::new(&mut config.packet_arrival_delay_ddos   , 0.01 ..=  20.).text("Packet Arrival Speed (DDoS)"));
            ui.add(egui::Slider::new(&mut config.packet_speed                , 0.   ..=  50.).text("Packet Speed"));
        }
    );
}
