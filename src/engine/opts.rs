use std::sync::RwLock;
use std::sync::TryLockError;

use anyhow::bail;
use anyhow::Result;
use vampirc_uci::UciOptionConfig;

use crate::debug::DebugLevel;
use crate::optlog;

#[inline(always)]
pub fn opts() -> Result<Opts> {
    match OPTS.try_read() {
        Ok(opts) => Ok(*opts),
        Err(e) => match e {
            TryLockError::Poisoned(e) => {
                bail!(
                    "Options should never be mutated during program execution.\n Error: {e}: {:?}",
                    e
                );
            }
            TryLockError::WouldBlock => {
                eprintln!("Options lock is blocked for reading. This should never happen.");
                match OPTS.read() {
                    Ok(opts) => Ok(*opts),
                    Err(e) => bail!(
                        "Options should never be mutated during program execution.\n Error: {e}: {:?}",
                        e
                    ),
                }
            }
        },
    }
}

pub fn setopts(opts: Opts) -> Result<()> {
    match OPTS.try_write() {
        Ok(mut o) => {
            *o = opts;
            Ok(())
        }
        Err(TryLockError::Poisoned(e)) => {
            bail!(
                "Options should never be mutated during program execution. Error: {e}:{:?}",
                e
            );
        }
        Err(TryLockError::WouldBlock) => {
            optlog!(opts;error;"Options lock is blocked. This should never happen.");
            match OPTS.write() {
                Ok(mut o) => {
                    *o = opts;
                    Ok(())
                }
                Err(e) => bail!(
                    "Options should never be mutated during program execution. Error: {e}:{:?}",
                    e
                ),
            }
        }
    }
}

pub static OPTS: RwLock<Opts> = RwLock::new(Opts::new());

/// Debug options for the engine
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Opts {
    pub search: DebugLevel,
    pub eval: DebugLevel,
    pub comm: DebugLevel,
    pub tt: DebugLevel,
    pub uci: DebugLevel,
    pub opts: DebugLevel,
    pub use_ab: bool,
    pub use_pv: bool,
    pub use_tt: bool,
    pub hash_size: usize,
}

impl Opts {
    /// By default, all debug levels are set to [`DebugLevel::info`],
    /// and all improvement options are enabled:
    /// * alpha-beta: true
    /// * use_pv: true
    ///
    /// [`Opts::default()`] will not enable any performance improvement options!
    ///
    /// Use [`Opts::new()`] instead.
    pub const fn new() -> Self {
        Self::initial().ab(true).pv(true).tt(true)
    }

    const fn initial() -> Self {
        Self {
            search: DebugLevel::info,
            eval: DebugLevel::info,
            comm: DebugLevel::info,
            tt: DebugLevel::info,
            uci: DebugLevel::info,
            opts: DebugLevel::debug,
            use_ab: false,
            use_pv: false,
            use_tt: false,
            hash_size: 16 * 1024,
        }
    }

    pub const fn bench() -> Self {
        Self {
            search: DebugLevel::off,
            eval: DebugLevel::off,
            comm: DebugLevel::off,
            tt: DebugLevel::off,
            uci: DebugLevel::off,
            opts: DebugLevel::error,
            use_ab: true,
            use_pv: true,
            use_tt: true,
            hash_size: 32,
        }
    }

    pub fn register_options() -> Vec<UciOptionConfig> {
        vec![
            UciOptionConfig::Check {
                name: "use_ab".to_string(),
                default: Some(true),
            },
            UciOptionConfig::Check {
                name: "use_pv".to_string(),
                default: Some(true),
            },
            UciOptionConfig::Check {
                name: "use_tt".to_string(),
                default: Some(true),
            },
            UciOptionConfig::Check {
                name: "bench_log".to_string(),
                default: Some(false),
            },
            UciOptionConfig::Spin {
                name: "search_debug".to_string(),
                default: Some(2),
                min: Some(0),
                max: Some(5),
            },
            UciOptionConfig::Spin {
                name: "eval_debug".to_string(),
                default: Some(2),
                min: Some(0),
                max: Some(5),
            },
            UciOptionConfig::Spin {
                name: "comm_debug".to_string(),
                default: Some(1),
                min: Some(0),
                max: Some(5),
            },
            UciOptionConfig::Spin {
                name: "tt_debug".to_string(),
                default: Some(1),
                min: Some(0),
                max: Some(5),
            },
            UciOptionConfig::Spin {
                name: "uci_debug".to_string(),
                default: Some(1),
                min: Some(0),
                max: Some(5),
            },
            UciOptionConfig::Spin {
                name: "hash".to_string(),
                default: Some(16),
                min: Some(0),
                max: Some(1024),
            },
        ]
    }

    pub fn receive_option(&mut self, name: &str, value: Option<&str>) -> Result<Self> {
        let parse_check = |check: &str, value: Option<&str>| match value.unwrap_or_default() {
            "on" => Ok(true),
            "off" => Ok(false),
            _ => bail!("you need to specify a value (on/off) for {check}"),
        };
        let parse_spin: fn(&str, i64, i64, Option<&str>) -> Result<i64> =
            |check: &str, low, high, value: Option<&str>| match value
                .unwrap_or_default()
                .parse::<i64>()
                .unwrap()
            {
                x if x <= high && x >= low => Ok(x),
                y if y > high => bail!("value {y} is too high for {check}. max allowed is {high}."),
                z if z < low => bail!("value {z} is too low for {check}. min allowed is {low}."),
                _ => unreachable!(),
            };
        match name {
            "use_ab" => self.use_ab = parse_check("use_ab", value)?,
            "use_pv" => self.use_pv = parse_check("use_pv", value)?,
            "use_tt" => self.use_tt = parse_check("use_tt", value)?,
            "bench_log" => {
                if parse_check("bench_log", value)? {
                    return Ok(Self::bench()
                        .ab(self.use_ab)
                        .pv(self.use_pv)
                        .tt(self.use_tt));
                }
            }
            "search_debug" => {
                self.search = DebugLevel::from(parse_spin("search_debug", 0, 5, value)?)
            }
            "eval_debug" => self.eval = DebugLevel::from(parse_spin("eval_debug", 0, 5, value)?),
            "comm_debug" => self.comm = DebugLevel::from(parse_spin("comm_debug", 0, 5, value)?),
            "tt_debug" => self.tt = DebugLevel::from(parse_spin("tt_debug", 0, 5, value)?),
            "uci_debug" => self.uci = DebugLevel::from(parse_spin("uci_debug", 0, 5, value)?),
            "hash" => self.hash_size = 1024 * parse_spin("hash", 0, 1024, value)? as usize,
            unknown => bail!("unknown option: {:?}", unknown),
        }

        Ok(*self)
    }

    /// Set the [`DebugLevel`] for *search*
    pub fn search(self, level: DebugLevel) -> Self {
        Self {
            search: level,
            ..self
        }
    }

    /// Set the [`DebugLevel`] for *eval*
    pub fn eval(self, level: DebugLevel) -> Self {
        Self {
            eval: level,
            ..self
        }
    }

    /// other_error[`DebugLevel`] for *communication* (between search and
    /// interface threads)
    pub fn comm(self, level: DebugLevel) -> Self {
        Self {
            comm: level,
            ..self
        }
    }

    /// Set the logging level for all modules
    pub fn db(self, level: DebugLevel) -> Self {
        Self {
            search: level,
            eval: level,
            comm: level,
            ..self
        }
    }

    /// Set the logging level for all modules
    pub fn debug(mut self, level: DebugLevel) -> Self {
        self.search = level;
        self.eval = level;
        self.comm = level;
        self
    }

    /// Enable or disable alpha-beta pruning during search
    pub const fn ab(self, x: bool) -> Self {
        Self { use_ab: x, ..self }
    }

    /// Enable or disable the use of the principal variation during search (for
    /// move ordering only)
    pub const fn pv(self, x: bool) -> Self {
        Self { use_pv: x, ..self }
    }

    /// Enable or disable the use of the transposition table during search
    pub const fn tt(self, x: bool) -> Self {
        Self { use_tt: x, ..self }
    }

    /// Set the transposition table size **in kilobytes**
    pub const fn hash_size(self, x: usize) -> Self {
        Self {
            hash_size: x,
            ..self
        }
    }
}
