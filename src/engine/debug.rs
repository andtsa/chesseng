use crate::DebugLevel;
use crate::Opts;

impl Opts {
    /// By standard, all debug levels are set to [`DebugLevel::Info`],
    /// and all improvement options are enabled:
    /// * alpha-beta: true
    /// * use_pv: true
    ///
    /// [`Opts::default()`] will not enable any performance improvement options!
    ///
    /// Use [`Opts::new()`] instead.
    pub fn new() -> Self {
        Self::default().ab(true).pv(true)
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

    /// Set the [`DebugLevel`] for *communication* (between search and interface
    /// threads)
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
    pub fn debug(&mut self, level: DebugLevel) {
        self.search = level;
        self.eval = level;
        self.comm = level;
    }

    /// Enable or disable alpha-beta pruning during search
    pub fn ab(self, x: bool) -> Self {
        Self { ab: x, ..self }
    }

    /// Enable or disable the use of the principal variation during search (for
    /// move ordering only)
    pub fn pv(self, x: bool) -> Self {
        Self { use_pv: x, ..self }
    }

    /// Search [`DebugLevel`] is TRACE
    pub fn st(&self) -> bool {
        self.search == DebugLevel::Trace
    }

    /// Search [`DebugLevel`] is DEBUG or higher (TRACE)
    pub fn sd(&self) -> bool {
        self.search == DebugLevel::Debug || self.st()
    }

    /// Search [`DebugLevel`] is INFO or higher (DEBUG, TRACE)
    pub fn si(&self) -> bool {
        self.search == DebugLevel::Info || self.sd()
    }

    /// Search [`DebugLevel`] is WARN or higher (INFO, DEBUG, TRACE)
    pub fn sw(&self) -> bool {
        self.search == DebugLevel::Warn || self.si()
    }

    /// Search [`DebugLevel`] is ERROR or higher (WARN, INFO, DEBUG, TRACE)
    pub fn se(&self) -> bool {
        self.search == DebugLevel::Error || self.sw()
    }

    /// Eval [`DebugLevel`] is TRACE
    pub fn et(&self) -> bool {
        self.eval == DebugLevel::Trace
    }

    /// Eval [`DebugLevel`] is DEBUG or higher (TRACE)
    pub fn ed(&self) -> bool {
        self.eval == DebugLevel::Debug || self.et()
    }

    /// Eval [`DebugLevel`] is INFO or higher (DEBUG, TRACE)
    pub fn ei(&self) -> bool {
        self.eval == DebugLevel::Info || self.ed()
    }

    /// Eval [`DebugLevel`] is WARN or higher (INFO, DEBUG, TRACE)
    pub fn ew(&self) -> bool {
        self.eval == DebugLevel::Warn || self.ei()
    }

    /// Eval [`DebugLevel`] is ERROR or higher (WARN, INFO, DEBUG, TRACE)
    pub fn ee(&self) -> bool {
        self.eval == DebugLevel::Error || self.ew()
    }

    /// Comm [`DebugLevel`] is TRACE
    pub fn ct(&self) -> bool {
        self.comm == DebugLevel::Trace
    }

    /// Comm [`DebugLevel`] is DEBUG or higher (TRACE)
    pub fn cd(&self) -> bool {
        self.comm == DebugLevel::Debug || self.ct()
    }

    /// Comm [`DebugLevel`] is INFO or higher (DEBUG, TRACE)
    pub fn ci(&self) -> bool {
        self.comm == DebugLevel::Info || self.cd()
    }

    /// Comm [`DebugLevel`] is WARN or higher (INFO, DEBUG, TRACE)
    pub fn cw(&self) -> bool {
        self.comm == DebugLevel::Warn || self.ci()
    }

    /// Comm [`DebugLevel`] is ERROR or higher (WARN, INFO, DEBUG, TRACE)
    pub fn ce(&self) -> bool {
        self.comm == DebugLevel::Error || self.cw()
    }

    /// if debug level, print the message
    pub fn edp(&self, msg: &str) {
        if self.ed() {
            println!("{}", msg);
        }
    }

    /// if trace level, print the message
    pub fn stp(&self, msg: &str) {
        if self.st() {
            println!("{}", msg);
        }
    }
}
