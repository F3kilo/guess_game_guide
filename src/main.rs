fn main() {
    let mut state = State::Menu;

    loop {
        let new_state = state.update();
        if let State::Exit = new_state {
            break;
        }

        state = new_state;
    }
}

enum State {
    Menu,
    Guessing(u32),
    Exit,
}

impl State {
    pub fn update(&self) -> Self {
        todo!()
    }
}