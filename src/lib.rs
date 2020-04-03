use string_fixed::StringFixed;

pub trait TraitGameData {
    fn initialize(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn audio(&mut self);
}

pub struct Game<D: TraitGameData, G> {
    name: StringFixed,
    frame_total: u32,
    pub data: D,
    pub gl: G,
}

impl<D: TraitGameData, G> Game<D, G> {
    pub fn initialize(&mut self, name: &str) {
        self.name.from_string(name);
        self.data.initialize();
    }
    pub fn update(&mut self) {
        self.data.update();
        self.frame_total = self.frame_total + 1;
    }
    pub fn render(&mut self) {
        self.data.render();
    }
    pub fn audio(&mut self) {
        self.data.audio();
    }
}
