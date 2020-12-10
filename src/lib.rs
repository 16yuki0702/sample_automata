use std::fmt;

#[derive(PartialEq, Debug)]
enum State {
    A,
    B,
    C,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::A => write!(f, "A"),
            State::B => write!(f, "B"),
            State::C => write!(f, "C"),
        }
    }
}

#[derive(Debug)]
struct DFA<'a> {
    input: &'a str,
    current_state: State,
}

impl<'a> DFA<'a> {
    fn new(input: &'a str) -> Self {
        DFA {
            input: input,
            current_state: State::A,
        }
    }

    fn transition(&mut self, c: char) {
        match self.current_state {
            State::A => match c {
                '0' => self.current_state = State::A,
                _ => self.current_state = State::B,
            },
            State::B => match c {
                '0' => self.current_state = State::C,
                _ => self.current_state = State::A,
            },
            State::C => match c {
                '0' => self.current_state = State::B,
                _ => self.current_state = State::C,
            },
        }
    }

    fn run(&mut self) -> &mut Self {
        for c in self.input.chars() {
            self.transition(c);
        }
        self
    }

    fn is_accepted(&self) -> bool {
        self.current_state == State::A
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfa() {
        assert_eq!(DFA::new("00000").run().is_accepted(), true);
        assert_eq!(DFA::new("0011").run().is_accepted(), true);
        assert_eq!(DFA::new("01110").run().is_accepted(), false);
        assert_eq!(DFA::new("1001").run().is_accepted(), true);
        assert_eq!(DFA::new("1110101110").run().is_accepted(), true);
    }
}
