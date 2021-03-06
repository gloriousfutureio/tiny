extern crate libc;
extern crate mio;
extern crate term_input;

use mio::Events;
use mio::Poll;
use mio::PollOpt;
use mio::Ready;
use mio::Token;
use mio::unix::EventedFd;

use term_input::{Event, Key, Input};

fn main() {
    // put the terminal in non-buffering, no-enchoing mode
    let mut old_term : libc::termios = unsafe { std::mem::zeroed() };
    unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut old_term); }

    let mut new_term : libc::termios = old_term.clone();
    new_term.c_iflag &= !(libc::IGNBRK | libc::BRKINT | libc::PARMRK | libc::ISTRIP | libc::INLCR |
                          libc::IGNCR | libc::ICRNL | libc::IXON);
    new_term.c_lflag &= !(libc::ICANON | libc::ECHO | libc::ISIG | libc::IEXTEN);
    unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &new_term) };

    let poll = Poll::new().unwrap();
    poll.register(
        &EventedFd(&libc::STDIN_FILENO),
        Token(libc::STDIN_FILENO as usize),
        Ready::readable(),
        PollOpt::level()).unwrap();

    let mut input = Input::new();
    let mut evs = Vec::new();
    let mut events = Events::with_capacity(10);
    'mainloop:
    loop {
        match poll.poll(&mut events, None) {
            Err(_) => {}
            Ok(_) => {
                input.read_input_events(&mut evs);
                println!("{:?}", evs);
                for ev in evs.iter() {
                    if ev == &Event::Key(Key::Esc) {
                        break 'mainloop;
                    }
                }
            }
        }
    }

    // restore the old settings
    unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &old_term) };
}
