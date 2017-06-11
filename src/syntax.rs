use std::mem::transmute;
use onig_sys;
use super::{SyntaxOperator, SyntaxBehavior, RegexOptions, MetaCharType};

pub enum MetaChar {
    Character(char),
    Ineffective,
}

#[derive(Debug, Clone, Copy)]
pub struct Syntax {
    raw: onig_sys::OnigSyntaxType,
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
        unsafe { transmute(onig_sys::OnigDefaultSyntax) }
    }

    pub fn operators(&self) -> SyntaxOperator {
        unsafe {
            let op = onig_sys::onig_get_syntax_op(&self.raw);
            let op2 = onig_sys::onig_get_syntax_op2(&self.raw);
            SyntaxOperator::from_bits_truncate(op as u64 + ((op2 as u64) << 32))
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
        let operators = self.operators() | operators;
        self.set_operators(operators)
    }

    pub fn disable_operators(&mut self, operators: SyntaxOperator) {
        let operators = self.operators() & !operators;
        self.set_operators(operators)
    }

    pub fn behavior(&self) -> SyntaxBehavior {
        SyntaxBehavior::from_bits_truncate(unsafe { onig_sys::onig_get_syntax_behavior(&self.raw) })
    }

    pub fn set_behavior(&mut self, behavior: SyntaxBehavior) {
        let behavior = behavior.bits() as onig_sys::OnigSyntaxBehavior;
        unsafe {
            onig_sys::onig_set_syntax_behavior(&mut self.raw, behavior);
        }
    }

    pub fn enable_behavior(&mut self, behavior: SyntaxBehavior) {
        let behavior = self.behavior() | behavior;
        self.set_behavior(behavior)
    }

    pub fn disable_behavior(&mut self, behavior: SyntaxBehavior) {
        let behavior = self.behavior() & !behavior;
        self.set_behavior(behavior)
    }

    pub fn options(&self) -> RegexOptions {
        RegexOptions::from_bits_truncate(unsafe { onig_sys::onig_get_syntax_options(&self.raw) })
    }

    pub fn set_options(&mut self, options: RegexOptions) {
        let options = options.bits() as onig_sys::OnigOptionType;
        unsafe {
            onig_sys::onig_set_syntax_options(&mut self.raw, options);
        }
    }

    pub fn set_meta_char(&mut self, what: MetaCharType, meta: MetaChar) {
        let what = what.bits();
        let code = match meta {
            MetaChar::Ineffective => onig_sys::ONIG_INEFFECTIVE_META_CHAR,
            MetaChar::Character(char) => char as u32,
        };
        unsafe {
            onig_sys::onig_set_meta_char(&mut self.raw, what, code);
        }
    }
}
