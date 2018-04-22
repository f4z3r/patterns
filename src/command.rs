//! Command design pattern.
//!
//! # Theory
//! This is a behavioural design pattern. It is used to encapsulate a request as an object, thereby letting you
//! parameterise clients with different requests, queue or log requests, and support undoable operations.
//!
//! In other words, it lets you turn the request itself into an object. It should be applied when it is necessary
//! to issue requests to objects without knowing anything about the operation being requested or the receiver of the
//! request. It decouples the object that invokes the operation from the one having the knowledge to perform it,
//! allowing more flexibility in the user interface.
//!
//! This supports _undo-redo_ operations. The command's `execute()` operation can store the initial state (or any
//! information required to invert the execution) for reversing its effect. To allow _undo_, a history list must store
//! all issued commands and an `unexecute()` operation must be supported by the `Command` interface. The prototype
//! pattern can be used to copy and store commands in the history list.
//!
//! This implements _transactional behaviour_ - a list of operations to be performed, if one operation fails, a rollback
//! is executed.
//!
//! # Participants
//! - `Command`: declares an interface for executing command (`execute()`).
//! - `TurnOnCommand`, `TurnOffCommand`: concrete commands specifying a receiver-action pair by storing the receiver as
//!   as a variable and invoking one or more operations on it when requested using `execute()`.
//! - `Switch`: the invoker. This contains a queue of commands that have been requested. This allows the _undo-redo_
//!   actions. It can decide when commands are executed and if they should be delayed.
//! - `Light`: the receiver. It knows how to perform the operations associated with carrying out a request.
//!
//! # Modifications and Strategies
//! - _Macro commands_ consisting of a sequence of elementary commands.
//! - Callbacks store the command objects somewhere to be called at a later point in time. Requests can be specified,
//!   queued, and executed at different times. The lifetime of the command object is then independent of the original
//!   request.
//! - _Wizards_: changes to be applied are stored and only executed once some `finish()` is called.
//! - One can modify undo-redo mechanisms to group operations together. For instance in a text editor, not every key
//!   press when writing something is undoable, instead they are undoable in reasonable batches.
//!
//! # Known Uses
//! - Wizard
//! - Undo-redo
//! - Log-files and re-execution
//! - Transactional Operations

use std::collections::VecDeque;

/// The command interface
trait Command {
    /// Executes the command
    fn execute(&self) -> &str;
}

/// The object the handle
#[derive(Clone, Copy)]
struct Light;

impl Light {
    fn turn_on(&self) -> &str {
        "light turned on"
    }

    fn turn_off(&self) -> &str {
        "light turned off"
    }
}

/// A concrete command
struct LightOnCommand {
    light: Light,
}

impl LightOnCommand {
    fn new(light: Light) -> Self {
        LightOnCommand {
            light,
        }
    }
}

impl Command for LightOnCommand {
    fn execute(&self) -> &str {
        self.light.turn_on()
    }
}

/// Another concrete command
struct LightOffCommand {
    light: Light,
}

impl LightOffCommand {
    fn new(light: Light) -> Self {
        LightOffCommand {
            light,
        }
    }
}

impl Command for LightOffCommand {
    fn execute(&self) -> &str {
        self.light.turn_off()
    }
}


/// A switch controlling the light
struct Switch<'a> {
    light: Light,
    history: VecDeque<Box<Command + 'a>>,
}

impl<'a> Switch<'a> {
    fn new() -> Switch<'a> {
        Switch {
            light: Light,
            history: VecDeque::new(),
        }
    }

    fn execute_command(&mut self, cmd: &str) -> &str {
        let command: Box<Command> = match cmd {
            "ON"    => Box::new(LightOnCommand::new(self.light)),
            "OFF"   => Box::new(LightOffCommand::new(self.light)),
            _       => panic!("Unexpected command"),
        };
        let result = match command.execute() {
            "light turned on"   => "light turned on",
            "light turned off"  => "light turned off",
            _                   => "unexpected result",
        };
        self.history.push_back(command);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let mut switch = Switch::new();
        assert_eq!(switch.execute_command("ON"), "light turned on");
        assert_eq!(switch.execute_command("OFF"), "light turned off");
        assert_eq!(switch.execute_command("ON"), "light turned on");
        assert_eq!(switch.execute_command("ON"), "light turned on");
    }

    #[test]
    #[should_panic(expected = "Unexpected command")]
    fn test_wrong_command() {
        let mut switch = Switch::new();
        let _ = switch.execute_command("Random_command");
    }
}
