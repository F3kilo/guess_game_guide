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
        match self {
            Self::Menu => Self::run_menu(),
            Self::Guessing(number) => Self::run_guessing(*number),
            Self::Exit => panic!("try to run Exit state"),
        }
    }

    fn run_menu() -> Self {
        todo!()
    }

    fn run_guessing(number: u32) -> Self {
        todo!()
    }
}