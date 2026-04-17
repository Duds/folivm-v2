
pub mod cell_render;
pub mod data_source;
pub mod export_hook;
pub mod panel;


pub trait Skill: Send + Sync {
    fn id(&self) -> String;
    fn name(&self) -> String;
}
