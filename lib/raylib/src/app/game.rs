use crate::app::AppContext;

pub trait Game {
    fn start(&mut self, ctx: AppContext);
    fn update(&mut self, ctx: AppContext) -> bool;
}
