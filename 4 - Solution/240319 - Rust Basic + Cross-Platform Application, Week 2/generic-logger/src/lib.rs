use std::fmt::Display;

pub trait Logger {
    // Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: impl Display);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: impl Display) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

fn do_things(logger: &impl Logger) {
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}

// Only log messages up to the given verbosity level.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: impl Display) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logger_low() {
        let log = VerbosityFilter {
            max_verbosity: 1,
            inner: StderrLogger,
        };
        do_things(&log);
    }

    #[test]
    fn logger_mid() {
        let log = VerbosityFilter {
            max_verbosity: 3,
            inner: StderrLogger,
        };
        do_things(&log);
    }

    #[test]
    fn logger_high() {
        let log = VerbosityFilter {
            max_verbosity: 6,
            inner: StderrLogger,
        };
        do_things(&log);
    }
}
