use crossterm::event::{self, KeyEvent, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode, KeyCode::Char, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    shoud_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { shoud_quit: false }
    }
    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!(
            "
            \n Goodbye  \r \n "
        );
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                );
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.shoud_quit = true;
                    }
                    _ => (),
                }
            }
            if self.shoud_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
