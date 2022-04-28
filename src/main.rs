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
        println!();
        println!("**** MENU ****");
        println!("1) Guessing");
        println!("Other) Exit");
        print!("Your choice: ");
        let choice = Self::read_input();
        match choice {
            Some(1) => Self::Guessing(Self::random_number()),
            _ => Self::Exit,
        }
    }

    fn read_input() -> Option<u32> {
        todo!()
    }

    fn random_number() -> u32 {
        todo!()
    }

    fn run_guessing(number: u32) -> Self {
        todo!()
    }
}
