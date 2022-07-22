pub trait System {
    fn name(&self) -> String;
    fn run(&mut self);
    fn init(&mut self);
    fn clean(&mut self);
}


#[derive(Debug, Clone)]
pub struct Systems<T: System + Clone + std::fmt::Debug + Send + 'static> {
    pub systems: Vec<T>
}
unsafe impl<T: System + Clone + std::fmt::Debug + Send + 'static> Send for Systems<T> {}
impl<T: System + Clone + std::fmt::Debug + Send + 'static> Systems<T> {
    pub fn new() -> Self {
        Self {
            systems: Vec::new()
        }
    }

    pub fn add(&mut self, system: T) {
        self.systems.push(system);
    }

    pub fn run(&mut self){
        for system in self.systems.iter_mut() {
            system.run();
        }
    }

    pub fn clean(&mut self){
        for system in self.systems.iter_mut() {
            system.clean();
        }
    }

    pub fn get(&mut self, name: String) -> Result<&T,&Vec<T>> {
        for system in &self.systems {
            if system.name() == name {
                return Ok(&system)
            };
        };
        Err(&self.systems)
    }
}

impl<T: System + Clone + std::fmt::Debug + Send + 'static> std::fmt::Display for Systems<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Systems: {:#?}", self.systems)
    }
}