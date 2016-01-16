use onig_sys::{OnigOptions, OnigSyntaxBehavior};

bitflags! {
    /// Regex parsing, compilation and evaluation options.
    flags Options: OnigOptions {
        /// Default options. This is both compile and search time option.
        const OPTION_NONE = 0,
        /// Ambiguity match on. This is compile time option.
        const OPTION_IGNORECASE = 1,
        /// Extended pattern form. This is compile time option.
        const OPTION_EXTEND = 2,
        /// `'.'` match with newline. This is compile time option.
        const OPTION_MULTILINE = 4,
        /// `'^'` -> `'\A'`, `'$'` -> `'\Z'`. This is compile time option.
        const OPTION_SINGLELINE = 8,
        /// Find longest match. This is compile time option.
        const OPTION_FIND_LONGEST = 16,
        /// Ignore empty match. This is compile time option.
        const OPTION_FIND_NOT_EMPTY = 32,
        /// Clear `OPTION_SINGLELINE` which is enabled on
        /// `SYNTAX_POSIX_BASIC`, `SYNTAX_POSIX_EXTENDED`,
        /// `SYNTAX_PERL`, `SYNTAX_PERL_NG`, `SYNTAX_JAVA`.
        /// This is compile time option.
        const OPTION_NEGATE_SINGLELINE = 64,
        /// Only named group captured. This is search time option.
        const OPTION_DONT_CAPTURE_GROUP = 128,
        /// Named and no-named group captured. This is search time option.
        const OPTION_CAPTURE_GROUP = 256,

        /// String head isn't considered as begin of line
        const OPTION_NOTBOL = 512,
        /// String end isn't considered as end of line
        const OPTION_NOTEOL = 1024,
        // const OPTION_POSIX_REGION = 2048,
        // const OPTION_MAXBIT = 4096
    }
}

bitflags! {
    flags SyntaxOperator: u64 {
        /// `.`
        const SYNTAX_OPERATOR_DOT_ANYCHAR                 = 1u64 << 1,
        /// `*`
        const SYNTAX_OPERATOR_ASTERISK_ZERO_INF           = 1u64 << 2,
        /// `+`
        const SYNTAX_OPERATOR_PLUS_ONE_INF                = 1u64 << 4,
        /// `?`
        const SYNTAX_OPERATOR_QMARK_ZERO_ONE              = 1u64 << 6,
        /// `{lower,upper}`
        const SYNTAX_OPERATOR_BRACE_INTERVAL              = 1u64 << 8,
        /// `\{lower,upper\}`
        const SYNTAX_OPERATOR_ESC_BRACE_INTERVAL          = 1u64 << 9,
        /// `|`
        const SYNTAX_OPERATOR_VBAR_ALT                    = 1u64 << 10,
        /// `\|`
        const SYNTAX_OPERATOR_ESC_VBAR_ALT                = 1u64 << 11,
        /// `(...)`
        const SYNTAX_OPERATOR_LPAREN_SUBEXP               = 1u64 << 12,
        /// `\(...\)`
        const SYNTAX_OPERATOR_ESC_LPAREN_SUBEXP           = 1u64 << 13,
        /// `\A, \Z, \z`
        const SYNTAX_OPERATOR_ESC_AZ_BUF_ANCHOR           = 1u64 << 14,
        /// `\G`
        const SYNTAX_OPERATOR_ESC_CAPITAL_G_BEGIN_ANCHOR  = 1u64 << 15,
        /// `\num`
        const SYNTAX_OPERATOR_DECIMAL_BACKREF             = 1u64 << 16,
        /// `[...]`
        const SYNTAX_OPERATOR_BRACKET_CC                  = 1u64 << 17,
        /// `\w, \W`
        const SYNTAX_OPERATOR_ESC_W_WORD                  = 1u64 << 18,
        /// `\<. \>`
        const SYNTAX_OPERATOR_ESC_LTGT_WORD_BEGIN_END     = 1u64 << 19,
        /// `\b, \B`
        const SYNTAX_OPERATOR_ESC_B_WORD_BOUND            = 1u64 << 20,
        /// `\s, \S`
        const SYNTAX_OPERATOR_ESC_S_WHITE_SPACE           = 1u64 << 21,
        /// `\d, \D`
        const SYNTAX_OPERATOR_ESC_D_DIGIT                 = 1u64 << 22,
        /// `^, $`
        const SYNTAX_OPERATOR_LINE_ANCHOR                 = 1u64 << 23,
        /// `[:xxxx:]`
        const SYNTAX_OPERATOR_POSIX_BRACKET               = 1u64 << 24,
        /// `??,*?,+?,{n,m}?`
        const SYNTAX_OPERATOR_QMARK_NON_GREEDY            = 1u64 << 25,
        /// `\n,\r,\t,\a ...`
        const SYNTAX_OPERATOR_ESC_CONTROL_CHARS           = 1u64 << 26,
        /// `\cx`
        const SYNTAX_OPERATOR_ESC_C_CONTROL               = 1u64 << 27,
        /// `\OOO`
        const SYNTAX_OPERATOR_ESC_OCTAL3                  = 1u64 << 28,
        /// `\xHH`
        const SYNTAX_OPERATOR_ESC_X_HEX2                  = 1u64 << 29,
        /// `\x{7HHHHHHH}`
        const SYNTAX_OPERATOR_ESC_X_BRACE_HEX8            = 1u64 << 30,
        /// `\Q...\E`
        const SYNTAX_OPERATOR_ESC_CAPITAL_Q_QUOTE         = 1u64 << (32 + 0),
        /// `(?...)`
        const SYNTAX_OPERATOR_QMARK_GROUP_EFFECT          = 1u64 << (32 + 1),
        /// `(?imsx),(?-imsx)`
        const SYNTAX_OPERATOR_OPTION_PERL                 = 1u64 << (32 + 2),
        /// `(?imx), (?-imx)`
        const SYNTAX_OPERATOR_OPTION_RUBY                 = 1u64 << (32 + 3),
        /// `?+,*+,++`
        const SYNTAX_OPERATOR_PLUS_POSSESSIVE_REPEAT      = 1u64 << (32 + 4),
        /// `{n,m}+`
        const SYNTAX_OPERATOR_PLUS_POSSESSIVE_INTERVAL    = 1u64 << (32 + 5),
        /// `[...&&..[..]..]`
        const SYNTAX_OPERATOR_CCLASS_SET_OP               = 1u64 << (32 + 6),
        /// `(?<name>...)`
        const SYNTAX_OPERATOR_QMARK_LT_NAMED_GROUP        = 1u64 << (32 + 7),
        /// `\k<name>`
        const SYNTAX_OPERATOR_ESC_K_NAMED_BACKREF         = 1u64 << (32 + 8),
        /// `\g<name>, \g<n>`
        const SYNTAX_OPERATOR_ESC_G_SUBEXP_CALL           = 1u64 << (32 + 9),
        /// `(?@..),(?@<x>..)`
        const SYNTAX_OPERATOR_ATMARK_CAPTURE_HISTORY      = 1u64 << (32 + 10),
        /// `\C-x`
        const SYNTAX_OPERATOR_ESC_CAPITAL_C_BAR_CONTROL   = 1u64 << (32 + 11),
        /// `\M-x`
        const SYNTAX_OPERATOR_ESC_CAPITAL_M_BAR_META      = 1u64 << (32 + 12),
        /// `\v as VTAB`
        const SYNTAX_OPERATOR_ESC_V_VTAB                  = 1u64 << (32 + 13),
        /// `\uHHHH`
        const SYNTAX_OPERATOR_ESC_U_HEX4                  = 1u64 << (32 + 14),
        /// `\`, \'`
        const SYNTAX_OPERATOR_ESC_GNU_BUF_ANCHOR          = 1u64 << (32 + 15),
        /// `\p{...}, \P{...}`
        const SYNTAX_OPERATOR_ESC_P_BRACE_CHAR_PROPERTY   = 1u64 << (32 + 16),
        /// `\p{^..}, \P{^..}`
        const SYNTAX_OPERATOR_ESC_P_BRACE_CIRCUMFLEX_NOT  = 1u64 << (32 + 17),
        /// `\h, \H`
        const SYNTAX_OPERATOR_ESC_H_XDIGIT                = 1u64 << (32 + 19),
        /// `\`
        const SYNTAX_OPERATOR_INEFFECTIVE_ESCAPE          = 1u64 << (32 + 20)
    }
}

bitflags! {
    flags SyntaxBehavior: OnigSyntaxBehavior {
        /// `?, *, +, {n,m}`
        const SYNTAX_BEHAVIOR_CONTEXT_INDEP_REPEAT_OPS        = 1u32 << 0,
        /// `error or ignore`
        const SYNTAX_BEHAVIOR_CONTEXT_INVALID_REPEAT_OPS      = 1u32 << 1,
        /// `...)...`
        const SYNTAX_BEHAVIOR_ALLOW_UNMATCHED_CLOSE_SUBEXP    = 1u32 << 2,
        /// `{???`
        const SYNTAX_BEHAVIOR_ALLOW_INVALID_INTERVAL          = 1u32 << 3,
        /// `{,n} => {0,n}`
        const SYNTAX_BEHAVIOR_ALLOW_INTERVAL_LOW_ABBREV       = 1u32 << 4,
        /// `/(\1)/,/\1()/ ..`
        const SYNTAX_BEHAVIOR_STRICT_CHECK_BACKREF            = 1u32 << 5,
        /// `(?<=a|bc)`
        const SYNTAX_BEHAVIOR_DIFFERENT_LEN_ALT_LOOK_BEHIND   = 1u32 << 6,
        /// See Oniguruma documenation
        const SYNTAX_BEHAVIOR_CAPTURE_ONLY_NAMED_GROUP        = 1u32 << 7,
        /// `(?<x>)(?<x>)`
        const SYNTAX_BEHAVIOR_ALLOW_MULTIPLEX_DEFINITION_NAME = 1u32 << 8,
        /// `a{n}?=(?:a{n})?`
        const SYNTAX_BEHAVIOR_FIXED_INTERVAL_IS_GREEDY_ONLY   = 1u32 << 9,
        /// `[^...]`
        const SYNTAX_BEHAVIOR_NOT_NEWLINE_IN_NEGATIVE_CC      = 1u32 << 20,
        /// `[..\w..] etc..`
        const SYNTAX_BEHAVIOR_BACKSLASH_ESCAPE_IN_CC          = 1u32 << 21,
        /// `[0-9-a]=[0-9\-a]`
        const SYNTAX_BEHAVIOR_ALLOW_DOUBLE_RANGE_OP_IN_CC     = 1u32 << 23,
        /// `[,-,]`
        const SYNTAX_BEHAVIOR_WARN_CC_OP_NOT_ESCAPED          = 1u32 << 24,
        /// `(?:a*)+`
        const SYNTAX_BEHAVIOR_WARN_REDUNDANT_NESTED_REPEAT    = 1u32 << 25
    }
}
