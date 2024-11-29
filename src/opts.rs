pub struct Opts {
    pub print_face_value: bool,
}

impl Opts {
    pub fn new(print_face_value: bool) -> Self {
        Self { print_face_value }
    }
}
