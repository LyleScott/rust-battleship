pub struct Ship {
    pub name: String,
    pub n_spaces: i8,
   // remaining_hits: i8,
}

impl Ship {
    pub fn get_label(&self) -> char {
        self.name.chars().nth(0).unwrap()
    }
}

pub fn ship_factory(name: String, n_spaces: i8) -> Ship {
    return Ship {
        name: name,
        n_spaces: n_spaces,
    }
}
