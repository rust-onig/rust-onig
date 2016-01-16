use std::mem::transmute;
use onig_sys;
use super::{SyntaxOperator, SyntaxBehavior, Options};

#[repr(C)]
#[derive(Debug)]
pub struct Syntax {
    raw: onig_sys::OnigSyntax
}

impl Clone for Syntax {
    fn clone(&self) -> Syntax {
        let mut syntax = Syntax {
            raw: onig_sys::OnigSyntax {
                op: 0,
                op2: 0,
                behavior: 0,
                options: 0,
                meta_char_table: onig_sys::OnigMetaCharTable {
                    esc: 0,
                    anychar: 0,
                    anytime: 0,
                    zero_or_one_time: 0,
                    one_or_more_time: 0,
                    anychar_anytime: 0,
                }
            }
        };
        Syntax::clone_from(&mut syntax, self);
        syntax
    }

    fn clone_from(&mut self, source: &Syntax) {
        unsafe {
            onig_sys::onig_copy_syntax(&mut self.raw, &source.raw)
        }
    }
}

impl Syntax {
    /// Plain text syntax
    pub fn asis() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxASIS) }
    }

    /// POSIX Basic RE syntax
    pub fn posix_basic() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxPosixBasic) }
    }

    /// POSIX Extended RE syntax
    pub fn posix_extended() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxPosixExtended) }
    }

    /// Emacs syntax
    pub fn emacs() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxEmacs) }
    }

    /// Grep syntax
    pub fn grep() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxGrep) }
    }

    /// GNU regex syntax
    pub fn gnu_regex() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxGnuRegex) }
    }

    /// Java (Sun java.util.regex) syntax
    pub fn java() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxJava) }
    }

    /// Perl syntax
    pub fn perl() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxPerl) }
    }

    /// Perl + named group syntax
    pub fn perl_ng() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxPerl_NG) }
    }

    /// Ruby syntax
    pub fn ruby() -> &'static Syntax {
        unsafe { transmute(&onig_sys::OnigSyntaxRuby) }
    }

    /// Default syntax (Ruby syntax)
    pub fn default() -> &'static Syntax {
        Syntax::ruby()  // TODO: use onig_get_default_syntax here
    }

    pub fn get_operators(&self) -> SyntaxOperator {
        unsafe {
            let op = onig_sys::onig_get_syntax_op(&self.raw);
            let op2 = onig_sys::onig_get_syntax_op2(&self.raw);
            SyntaxOperator::from_bits_truncate(
                op as u64 + ((op2 as u64) << 32)
            )
        }
    }

    pub fn set_operators(&mut self, operators: SyntaxOperator) {
        let op = operators.bits() as onig_sys::OnigSyntaxOp;
        let op2 = (operators.bits() >> 32) as onig_sys::OnigSyntaxOp2;
        unsafe {
            onig_sys::onig_set_syntax_op(&mut self.raw, op);
            onig_sys::onig_set_syntax_op2(&mut self.raw, op2)
        }
    }

    pub fn enable_operators(&mut self, operators: SyntaxOperator) {
        let operators = self.get_operators() | operators;
        self.set_operators(operators)
    }

    pub fn disable_operators(&mut self, operators: SyntaxOperator) {
        let operators = self.get_operators() & !operators;
        self.set_operators(operators)
    }

    pub fn get_behavior(&self) -> SyntaxBehavior {
        SyntaxBehavior::from_bits_truncate(unsafe {
            onig_sys::onig_get_syntax_behavior(&self.raw)
        })
    }

    pub fn set_behavior(&mut self, behavior: SyntaxBehavior) {
        let behavior = behavior.bits() as onig_sys::OnigSyntaxBehavior;
        unsafe {
            onig_sys::onig_set_syntax_behavior(&mut self.raw, behavior);
        }
    }

    pub fn enable_behavior(&mut self, behavior: SyntaxBehavior) {
        let behavior = self.get_behavior() | behavior;
        self.set_behavior(behavior)
    }

    pub fn disable_behavior(&mut self, behavior: SyntaxBehavior) {
        let behavior = self.get_behavior() & !behavior;
        self.set_behavior(behavior)
    }

    pub fn get_options(&self) -> Options {
        Options::from_bits_truncate(unsafe {
            onig_sys::onig_get_syntax_options(&self.raw)
        })
    }

    pub fn set_options(&mut self, options: Options) {
        let options = options.bits() as onig_sys::OnigOptions;
        unsafe {
            onig_sys::onig_set_syntax_options(&mut self.raw, options);
        }
    }
}
