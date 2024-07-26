use ::bevy::prelude::*;
use ::bevy_egui::{egui, EguiContexts};
use crate::{Configs::World::WorldConfig, Levels::Hardware::Disk::Disk_Buy};



pub struct PlayerPlugin;
impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut bevy::prelude::App)
    {
        app.init_resource::<PlayerConfig>();
        app.add_systems(Update, player_ui);
    }
}


#[derive(Resource)]
pub struct PlayerConfig
{
    pub money    : usize,
    pub antivirus: f32,
    pub firewall : f32,
    pub layer    : Layer,
}
impl Default for PlayerConfig
{
    fn default() -> Self {
        Self {
            money    : 200 ,
            antivirus:   0.,
            firewall :   0.,
            layer    : Layer::Company,
        }
    }
}
impl PlayerConfig
{
    pub fn get_money    (&    self) -> &    usize {&    self.money}
    pub fn mut_money    (&mut self) -> &mut usize {&mut self.money}
    pub fn get_antivirus(&    self) -> &    f32   {&    self.antivirus}
    pub fn get_firewall (&    self) -> &    f32   {&    self.firewall}
    pub fn get_layer    (&    self) -> &    Layer {&    self.layer}
    pub fn mut_layer    (&mut self) -> &mut Layer {&mut self.layer}
}
#[derive(PartialEq)]
pub enum Layer
{
    Software,
    Hardware,
    Company,
}



pub fn player_ui(
    // mut commands: Commands,
    mut contexts: EguiContexts,
    mut player  : ResMut<PlayerConfig>,
    world: Res<WorldConfig>,
    mut writer_disk: EventWriter<Disk_Buy>,
){
    egui::Window::new("Player Control").show(
        contexts.ctx_mut(),
        |ui|
        {
            ui.add(egui::Label::new(player.get_money().to_string()));
            if ui.button("Buy Antivirus").clicked() {
                if player.get_money() >= &world.get_antivirus_cost() {
                    *player.mut_money() -= world.get_antivirus_cost();
                    player.antivirus = world.get_antivirus_time();
                }
            }
            if ui.button("Buy Firewall").clicked() {
                if player.get_money() >= &world.get_firewall_cost() {
                    *player.mut_money() -= world.get_firewall_cost();
                    player.firewall = world.get_firewall_time();
                }
            }
            if ui.button("Buy Disk").clicked() {
                if player.get_money() >= &world.get_disk_cost() {
                    *player.mut_money() -= world.get_disk_cost();
                    writer_disk.send(Disk_Buy {});
                }
            }
            ui.label("Select Layer:");
            ui.selectable_value(player.mut_layer(), Layer::Software, "Software-Layer");
            ui.selectable_value(player.mut_layer(), Layer::Hardware, "Hardware-Layer");
            ui.selectable_value(player.mut_layer(), Layer::Company , "Company-Layer");
            // ui.add(egui::Slider::new(&mut config.packet_arrival_delay, 0.01..=10.0).text("Packet Arrival Speed"));
        }
    );
}
