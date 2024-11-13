#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(non_camel_case_types)] // to call in macros with lowercase
pub enum DebugLevel {
    off,
    /// !!!! warning: importing this directly
    /// (`use crate::debug::DebugLevel::error`) messes up `anyhow` macros
    error,
    warn,
    #[default]
    info,
    debug,
    trace,
}

#[macro_export]
macro_rules! optlog {
    ($module:ident;$level:ident;$($arg:tt)*) => {
        {
            if $crate::opts::opts().unwrap().$module.$level() {
                log::$level!("{}: {}", stringify!($module), format!($($arg)*));
            }
        }
    };
}

// /// Querying the options, shorthand for discreet use in code. don't worry
// about /// this too much, kinda garbage code xd
// impl Opts {
//     /// Search [`DebugLevel`] is TRACE
//     pub fn st(&self) -> bool {
//         self.search == DebugLevel::trace
//     }
//
//     /// Search [`DebugLevel`] is DEBUG or higher (TRACE)
//     pub fn sd(&self) -> bool {
//         self.search == DebugLevel::debug || self.st()
//     }
//
//     /// Search [`DebugLevel`] is INFO or higher (DEBUG, TRACE)
//     pub fn si(&self) -> bool {
//         self.search == DebugLevel::info || self.sd()
//     }
//
//     /// Search [`DebugLevel`] is WARN or higher (INFO, DEBUG, TRACE)
//     pub fn sw(&self) -> bool {
//         self.search == DebugLevel::warn || self.si()
//     }
//
//     /// Search [`DebugLevel`] is ERROR or higher (WARN, INFO, DEBUG, TRACE)
//     pub fn se(&self) -> bool {
//         self.search == DebugLevel::error || self.sw()
//     }
//
//     /// Eval [`DebugLevel`] is TRACE
//     pub fn et(&self) -> bool {
//         self.eval == DebugLevel::trace
//     }
//
//     /// Eval [`DebugLevel`] is DEBUG or higher (TRACE)
//     pub fn ed(&self) -> bool {
//         self.eval == DebugLevel::debug || self.et()
//     }
//
//     /// Eval [`DebugLevel`] is INFO or higher (DEBUG, TRACE)
//     pub fn ei(&self) -> bool {
//         self.eval == DebugLevel::info || self.ed()
//     }
//
//     /// Eval [`DebugLevel`] is WARN or higher (INFO, DEBUG, TRACE)
//     pub fn ew(&self) -> bool {
//         self.eval == DebugLevel::warn || self.ei()
//     }
//
//     /// Eval [`DebugLevel`] is ERROR or higher (WARN, INFO, DEBUG, TRACE)
//     pub fn ee(&self) -> bool {
//         self.eval == DebugLevel::error || self.ew()
//     }
//
//     /// Comm [`DebugLevel`] is TRACE
//     pub fn ct(&self) -> bool {
//         self.comm == DebugLevel::trace
//     }
//
//     /// Comm [`DebugLevel`] is DEBUG or higher (TRACE)
//     pub fn cd(&self) -> bool {
//         self.comm == DebugLevel::debug || self.ct()
//     }
//
//     /// Comm [`DebugLevel`] is INFO or higher (DEBUG, TRACE)
//     pub fn ci(&self) -> bool {
//         self.comm == DebugLevel::info || self.cd()
//     }
//
//     /// Comm [`DebugLevel`] is WARN or higher (INFO, DEBUG, TRACE)
//     pub fn cw(&self) -> bool {
//         self.comm == DebugLevel::warn || self.ci()
//     }
//
//     /// Comm [`DebugLevel`] is ERROR or higher (WARN, INFO, DEBUG, TRACE)
//     pub fn ce(&self) -> bool {
//         self.comm == DebugLevel::error || self.cw()
//     }
//
//     /// if debug level, print the message
//     pub fn edp(&self, msg: &str) {
//         if self.ed() {
//             println!("{}", msg);
//         }
//     }
//
//     /// if trace level, print the message
//     pub fn stp(&self, msg: &str) {
//         if self.st() {
//             println!("{}", msg);
//         }
//     }
//
//     pub fn ut(&self) -> bool {
//         self.uci == DebugLevel::trace
//     }
//
//     pub fn ud(&self) -> bool {
//         self.uci == DebugLevel::debug || self.ut()
//     }
// }

impl DebugLevel {
    /// This [`DebugLevel`] is TRACE
    #[inline(always)]
    pub fn trace(&self) -> bool {
        *self == Self::trace
    }

    /// This [`DebugLevel`] is DEBUG or higher (TRACE)
    #[inline(always)]
    pub fn debug(&self) -> bool {
        *self == Self::debug || self.trace()
    }

    /// This [`DebugLevel`] is INFO or higher (DEBUG, TRACE)
    #[inline(always)]
    pub fn info(&self) -> bool {
        *self == Self::info || self.debug()
    }

    /// This [`DebugLevel`] is WARN or higher (INFO, DEBUG, TRACE)
    #[inline(always)]
    pub fn warn(&self) -> bool {
        *self == Self::warn || self.info()
    }

    /// This [`DebugLevel`] is ERROR or higher (WARN, INFO, DEBUG, TRACE)
    #[inline(always)]
    pub fn error(&self) -> bool {
        *self == Self::error || self.warn()
    }

    /// This [`DebugLevel`] is OFF
    #[inline(always)]
    pub fn off(&self) -> bool {
        *self == Self::off
    }
}

impl From<i64> for DebugLevel {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::off,
            1 => Self::error,
            2 => Self::warn,
            3 => Self::info,
            4 => Self::debug,
            5 => Self::trace,
            _ => panic!("invalid value for DebugLevel: {}", value),
        }
    }
}
