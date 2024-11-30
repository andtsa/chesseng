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

/// # `optlog!`
/// ###### optional logging based on the debug level from global options
/// examples:
/// ```rust
/// # use sandy_engine::optlog;
/// // suppose we are doing some calculations
/// // in the evaluation function...
/// let x = 5;
/// optlog!(eval;trace;"x is {}", x);
/// // this will only print if the global option for `eval` is set to trace,
/// // and debug_assertions is turned on
/// ```
/// ```rust
/// # use sandy_engine::optlog;
/// // in the universal chess interface...
/// optlog!(uci;error;"not allowed!");
/// // this will only *not* print if the global option for `uci` is set to `off`,
/// ```
#[macro_export]
macro_rules! optlog {
    ($module:ident;$level:ident;$($arg:tt)*) => {
        {
            match $crate::debug::DebugLevel::$level {
                $crate::debug::DebugLevel::debug | $crate::debug::DebugLevel::trace => {
                    #[cfg(debug_assertions)]
                    if $crate::opts::opts().unwrap().$module.$level() {
                        log::$level!("{}: {}", stringify!($module), format!($($arg)*));
                    }
                }
                _ => {
                    if $crate::opts::opts().unwrap().$module.$level() {
                        log::$level!("{}: {}", stringify!($module), format!($($arg)*));
                    }
                }
            }


        }
    };
}

#[macro_export]
macro_rules! primary {
    ($module:ident;$level:ident;$($arg:tt)*) => {
        #[cfg(not(debug_assertions))]
        compile_error!(concat!(
            "error: forgotten primary!() macro call\n",
            "  --> ", file!(), ":", line!(), ":1\n",
            "   |\n",
            "   = note: in module `", stringify!($module), "`"
        ));
        #[cfg(debug_assertions)]
        println!(
            "[PRIMARY] {}: {}",
            stringify!($module),
            format!($($arg)*)
        );
    };
}

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
