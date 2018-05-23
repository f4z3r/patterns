//! Façade Design pattern.
//!
//! # Theory
//! Strucutural design pattern providing a unifided interface to a set of interfaces in a subsystem. It defines a higher
//! level interface that makes the subsystem easier to use. It is often used by implementors when the subsystem is very
//! complex or difficult to understand because the system has a large number of interdependent classes or because its
//! source code is unavailable. A good example for this is a compiler. Compilers are composed of parsers, code
//! generators, linkers, optimisors, etc. However, in most cases, clients simply want to compile some source code. Hence
//! the compiler has a facade interface that aggregates the calls to the subsystem components in order to provide a
//! simple call to parse, generate code, optimise it, and then link it to existing libraries in a single go. This allows
//! to make life easier for most programmers without hiding low-level functionality from the ones that require it.
//!
//! # Participants
//! - `Compiler`: the facade for all its components. It knows which subsystem classes are responsible for a request. It
//!    then delegates client requests to appropriate subsystem objects.
//! - `Linker`, `Parser`, `CodeGenerator` and `Optimiser`: subsystem classes implementing subsystem functionality. They
//!    handle the work assigned by the façade object. They may have no knowledge of the façade (as in the implementation
//!    below).
//!
//! # Modifications and Strategies
//! The coupling between clients and the subsytem can be reduced even further by making the façade an abstract class (or
//! a trait in Rust) with concrete subclasses (or trait implementations) for different implementations of a subsystem.
//!
//! # Known Uses
//! - Compilers
//! - Any library providing a simplified "general purpose" interface for a more complex underlying code base.

/// A subsystem.
struct Linker;
impl Linker {
    fn run(&self) -> &str {
        "linking code"
    }
}

/// Another subsystem.
struct Parser;
impl Parser {
    fn run(&self) -> &str {
        "parsing source code"
    }
}

/// Another subsystem.
struct Optimiser;
impl Optimiser {
    fn run(&self) -> &str {
        "optimising generate machine code"
    }
}

/// Another subsystem.
struct CodeGenerator;
impl CodeGenerator {
    fn run(&self) -> &str {
        "generating machine code"
    }
}

/// The entire system. A facade for all its subsystem components.
struct Compiler {
    parser: Parser,
    generator: CodeGenerator,
    optimiser: Optimiser,
    linker: Linker,
}
impl Compiler {
    fn new() -> Compiler {
        Compiler {
            parser: Parser,
            generator: CodeGenerator,
            optimiser: Optimiser,
            linker: Linker,
        }
    }

    fn run(&self) -> String {
        format!("{}\n{}\n{}\n{}",
                self.parser.run(),
                self.generator.run(),
                self.optimiser.run(),
                self.linker.run())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facade() {
        let compiler = Compiler::new();
        let mut exp_result = String::from("parsing source code");
        exp_result.push_str("\ngenerating machine code");
        exp_result.push_str("\noptimising generate machine code");
        exp_result.push_str("\nlinking code");

        assert_eq!(compiler.run(), exp_result);
    }
}
