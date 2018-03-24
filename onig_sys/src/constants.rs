use libc::{c_int, c_uint};
use super::{OnigCaseFoldType, OnigCtype, OnigDistance, OnigOptionType, OnigSyntaxBehavior,
            OnigSyntaxOp, OnigSyntaxOp2};

macro_rules! define_consts {
    ($typ:ty, $( $name:ident = $value:expr );* ) => {
        $(pub const $name: $typ = $value;)*
    };
}

// Character types
define_consts!(OnigCtype,
    ONIGENC_CTYPE_NEWLINE = 0;
    ONIGENC_CTYPE_ALPHA   = 1;
    ONIGENC_CTYPE_BLANK   = 2;
    ONIGENC_CTYPE_CNTRL   = 3;
    ONIGENC_CTYPE_DIGIT   = 4;
    ONIGENC_CTYPE_GRAPH   = 5;
    ONIGENC_CTYPE_LOWER   = 6;
    ONIGENC_CTYPE_PRINT   = 7;
    ONIGENC_CTYPE_PUNCT   = 8;
    ONIGENC_CTYPE_SPACE   = 9;
    ONIGENC_CTYPE_UPPER   = 10;
    ONIGENC_CTYPE_XDIGIT  = 11;
    ONIGENC_CTYPE_WORD    = 12;
    ONIGENC_CTYPE_ALNUM   = 13;  /* alpha || digit */
    ONIGENC_CTYPE_ASCII   = 14;
    ONIGENC_MAX_STD_CTYPE = ONIGENC_CTYPE_ASCII
);

// Options
define_consts!(OnigOptionType,
// Compile time
    ONIG_OPTION_DEFAULT            = ONIG_OPTION_NONE;
    ONIG_OPTION_NONE               = 0;
    ONIG_OPTION_IGNORECASE         = 1;
    ONIG_OPTION_EXTEND             = ONIG_OPTION_IGNORECASE         << 1;
    ONIG_OPTION_MULTILINE          = ONIG_OPTION_EXTEND             << 1;
    ONIG_OPTION_SINGLELINE         = ONIG_OPTION_MULTILINE          << 1;
    ONIG_OPTION_FIND_LONGEST       = ONIG_OPTION_SINGLELINE         << 1;
    ONIG_OPTION_FIND_NOT_EMPTY     = ONIG_OPTION_FIND_LONGEST       << 1;
    ONIG_OPTION_NEGATE_SINGLELINE  = ONIG_OPTION_FIND_NOT_EMPTY     << 1;
    ONIG_OPTION_DONT_CAPTURE_GROUP = ONIG_OPTION_NEGATE_SINGLELINE  << 1;
    ONIG_OPTION_CAPTURE_GROUP      = ONIG_OPTION_DONT_CAPTURE_GROUP << 1;
// Search time
    ONIG_OPTION_NOTBOL             = ONIG_OPTION_CAPTURE_GROUP      << 1;
    ONIG_OPTION_NOTEOL             = ONIG_OPTION_NOTBOL             << 1;
    ONIG_OPTION_POSIX_REGION       = ONIG_OPTION_NOTEOL             << 1;
    ONIG_OPTION_MAXBIT             = ONIG_OPTION_POSIX_REGION  /* limit */
);

// Syntax operators
define_consts!(OnigSyntaxOp,
    ONIG_SYN_OP_VARIABLE_META_CHARACTERS   = 1 << 0;
    ONIG_SYN_OP_DOT_ANYCHAR                = 1 << 1;   /* . */
    ONIG_SYN_OP_ASTERISK_ZERO_INF          = 1 << 2;   /* * */
    ONIG_SYN_OP_ESC_ASTERISK_ZERO_INF      = 1 << 3;
    ONIG_SYN_OP_PLUS_ONE_INF               = 1 << 4;   /* + */
    ONIG_SYN_OP_ESC_PLUS_ONE_INF           = 1 << 5;
    ONIG_SYN_OP_QMARK_ZERO_ONE             = 1 << 6;   /* ? */
    ONIG_SYN_OP_ESC_QMARK_ZERO_ONE         = 1 << 7;
    ONIG_SYN_OP_BRACE_INTERVAL             = 1 << 8;   /* {lower,upper} */
    ONIG_SYN_OP_ESC_BRACE_INTERVAL         = 1 << 9;   /* \{lower,upper\} */
    ONIG_SYN_OP_VBAR_ALT                   = 1 << 10;   /* | */
    ONIG_SYN_OP_ESC_VBAR_ALT               = 1 << 11;  /* \| */
    ONIG_SYN_OP_LPAREN_SUBEXP              = 1 << 12;  /* (...)   */
    ONIG_SYN_OP_ESC_LPAREN_SUBEXP          = 1 << 13;  /* \(...\) */
    ONIG_SYN_OP_ESC_AZ_BUF_ANCHOR          = 1 << 14;  /* \A, \Z, \z */
    ONIG_SYN_OP_ESC_CAPITAL_G_BEGIN_ANCHOR = 1 << 15;  /* \G     */
    ONIG_SYN_OP_DECIMAL_BACKREF            = 1 << 16;  /* \num   */
    ONIG_SYN_OP_BRACKET_CC                 = 1 << 17;  /* [...]  */
    ONIG_SYN_OP_ESC_W_WORD                 = 1 << 18;  /* \w, \W */
    ONIG_SYN_OP_ESC_LTGT_WORD_BEGIN_END    = 1 << 19;  /* \<. \> */
    ONIG_SYN_OP_ESC_B_WORD_BOUND           = 1 << 20;  /* \b, \B */
    ONIG_SYN_OP_ESC_S_WHITE_SPACE          = 1 << 21;  /* \s, \S */
    ONIG_SYN_OP_ESC_D_DIGIT                = 1 << 22;  /* \d, \D */
    ONIG_SYN_OP_LINE_ANCHOR                = 1 << 23;  /* ^, $   */
    ONIG_SYN_OP_POSIX_BRACKET              = 1 << 24;  /* [:xxxx:] */
    ONIG_SYN_OP_QMARK_NON_GREEDY           = 1 << 25;  /* ??,*?,+?,{n,m}? */
    ONIG_SYN_OP_ESC_CONTROL_CHARS          = 1 << 26;  /* \n,\r,\t,\a ... */
    ONIG_SYN_OP_ESC_C_CONTROL              = 1 << 27;  /* \cx  */
    ONIG_SYN_OP_ESC_OCTAL3                 = 1 << 28;  /* \OOO */
    ONIG_SYN_OP_ESC_X_HEX2                 = 1 << 29;  /* \xHH */
    ONIG_SYN_OP_ESC_X_BRACE_HEX8           = 1 << 30   /* \x{7HHHHHHH} */
);

// Syntax operators 2
define_consts!(OnigSyntaxOp2,
    ONIG_SYN_OP2_ESC_CAPITAL_Q_QUOTE        = 1 << 0;  /* \Q...\E */
    ONIG_SYN_OP2_QMARK_GROUP_EFFECT         = 1 << 1;  /* (?...) */
    ONIG_SYN_OP2_OPTION_PERL                = 1 << 2;  /* (?imsx),(?-imsx) */
    ONIG_SYN_OP2_OPTION_RUBY                = 1 << 3;  /* (?imx), (?-imx)  */
    ONIG_SYN_OP2_PLUS_POSSESSIVE_REPEAT     = 1 << 4;  /* ?+,*+,++ */
    ONIG_SYN_OP2_PLUS_POSSESSIVE_INTERVAL   = 1 << 5;  /* {n,m}+   */
    ONIG_SYN_OP2_CCLASS_SET_OP              = 1 << 6;  /* [...&&..[..]..] */
    ONIG_SYN_OP2_QMARK_LT_NAMED_GROUP       = 1 << 7;  /* (?<name>...) */
    ONIG_SYN_OP2_ESC_K_NAMED_BACKREF        = 1 << 8;  /* \k<name> */
    ONIG_SYN_OP2_ESC_G_SUBEXP_CALL          = 1 << 9;  /* \g<name>, \g<n> */
    ONIG_SYN_OP2_ATMARK_CAPTURE_HISTORY     = 1 << 10; /* (?@..),(?@<x>..) */
    ONIG_SYN_OP2_ESC_CAPITAL_C_BAR_CONTROL  = 1 << 11; /* \C-x */
    ONIG_SYN_OP2_ESC_CAPITAL_M_BAR_META     = 1 << 12; /* \M-x */
    ONIG_SYN_OP2_ESC_V_VTAB                 = 1 << 13; /* \v as VTAB */
    ONIG_SYN_OP2_ESC_U_HEX4                 = 1 << 14; /* \uHHHH */
    ONIG_SYN_OP2_ESC_GNU_BUF_ANCHOR         = 1 << 15; /* \`, \' */
    ONIG_SYN_OP2_ESC_P_BRACE_CHAR_PROPERTY  = 1 << 16; /* \p{...}, \P{...} */
    ONIG_SYN_OP2_ESC_P_BRACE_CIRCUMFLEX_NOT = 1 << 17; /* \p{^..}, \P{^..} */
    ONIG_SYN_OP2_ESC_H_XDIGIT               = 1 << 19; /* \h, \H */
    ONIG_SYN_OP2_INEFFECTIVE_ESCAPE         = 1 << 20 /* \ */
);

// Syntax behavior
define_consts!(OnigSyntaxBehavior,
    ONIG_SYN_CONTEXT_INDEP_ANCHORS           = 1 << 31; /* not implemented */
    ONIG_SYN_CONTEXT_INDEP_REPEAT_OPS        = 1 << 0;  /* ?, *, +, {n,m} */
    ONIG_SYN_CONTEXT_INVALID_REPEAT_OPS      = 1 << 1;  /* error or ignore */
    ONIG_SYN_ALLOW_UNMATCHED_CLOSE_SUBEXP    = 1 << 2;  /* ...)... */
    ONIG_SYN_ALLOW_INVALID_INTERVAL          = 1 << 3;  /* {??? */
    ONIG_SYN_ALLOW_INTERVAL_LOW_ABBREV       = 1 << 4;  /* {,n} => {0,n} */
    ONIG_SYN_STRICT_CHECK_BACKREF            = 1 << 5;  /* /(\1)/,/\1()/ ..*/
    ONIG_SYN_DIFFERENT_LEN_ALT_LOOK_BEHIND   = 1 << 6;  /* (?<=a|bc) */
    ONIG_SYN_CAPTURE_ONLY_NAMED_GROUP        = 1 << 7;  /* see doc/RE */
    ONIG_SYN_ALLOW_MULTIPLEX_DEFINITION_NAME = 1 << 8;  /* (?<x>)(?<x>) */
    ONIG_SYN_FIXED_INTERVAL_IS_GREEDY_ONLY   = 1 << 9;  /* a{n}?=(?:a{n})? */
    ONIG_SYN_NOT_NEWLINE_IN_NEGATIVE_CC      = 1 << 20; /* [^...] */
    ONIG_SYN_BACKSLASH_ESCAPE_IN_CC          = 1 << 21; /* [..\w..] etc.. */
    ONIG_SYN_ALLOW_EMPTY_RANGE_IN_CC         = 1 << 22;
    ONIG_SYN_ALLOW_DOUBLE_RANGE_OP_IN_CC     = 1 << 23; /* [0-9-a]=[0-9\-a] */
    ONIG_SYN_WARN_CC_OP_NOT_ESCAPED          = 1 << 24; /* [,-,] */
    ONIG_SYN_WARN_REDUNDANT_NESTED_REPEAT    = 1 << 25 /* (?:a*)+ */
);

// Returns
define_consts!(c_int,
    ONIG_NORMAL = 0;
    ONIG_MISMATCH = -1;
    ONIG_NO_SUPPORT_CONFIG = -2;
    ONIGERR_MEMORY = -5;
    ONIGERR_TYPE_BUG = -6;
    ONIGERR_PARSER_BUG = -11;
    ONIGERR_STACK_BUG = -12;
    ONIGERR_UNDEFINED_BYTECODE = -13;
    ONIGERR_UNEXPECTED_BYTECODE = -14;
    ONIGERR_MATCH_STACK_LIMIT_OVER = -15;
    ONIGERR_DEFAULT_ENCODING_IS_NOT_SETTED = -21;
    ONIGERR_SPECIFIED_ENCODING_CANT_CONVERT_TO_WIDE_CHAR = -22;
    ONIGERR_INVALID_ARGUMENT = -30;
    ONIGERR_END_PATTERN_AT_LEFT_BRACE = -100;
    ONIGERR_END_PATTERN_AT_LEFT_BRACKET = -101;
    ONIGERR_EMPTY_CHAR_CLASS = -102;
    ONIGERR_PREMATURE_END_OF_CHAR_CLASS = -103;
    ONIGERR_END_PATTERN_AT_ESCAPE = -104;
    ONIGERR_END_PATTERN_AT_META = -105;
    ONIGERR_END_PATTERN_AT_CONTROL = -106;
    ONIGERR_META_CODE_SYNTAX = -108;
    ONIGERR_CONTROL_CODE_SYNTAX = -109;
    ONIGERR_CHAR_CLASS_VALUE_AT_END_OF_RANGE = -110;
    ONIGERR_CHAR_CLASS_VALUE_AT_START_OF_RANGE = -111;
    ONIGERR_UNMATCHED_RANGE_SPECIFIER_IN_CHAR_CLASS = -112;
    ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED = -113;
    ONIGERR_TARGET_OF_REPEAT_OPERATOR_INVALID = -114;
    ONIGERR_NESTED_REPEAT_OPERATOR = -115;
    ONIGERR_UNMATCHED_CLOSE_PARENTHESIS = -116;
    ONIGERR_END_PATTERN_WITH_UNMATCHED_PARENTHESIS = -117;
    ONIGERR_END_PATTERN_IN_GROUP = -118;
    ONIGERR_UNDEFINED_GROUP_OPTION = -119;
    ONIGERR_INVALID_POSIX_BRACKET_TYPE = -121;
    ONIGERR_INVALID_LOOK_BEHIND_PATTERN = -122;
    ONIGERR_INVALID_REPEAT_RANGE_PATTERN = -123;
    ONIGERR_TOO_BIG_NUMBER = -200;
    ONIGERR_TOO_BIG_NUMBER_FOR_REPEAT_RANGE = -201;
    ONIGERR_UPPER_SMALLER_THAN_LOWER_IN_REPEAT_RANGE = -202;
    ONIGERR_EMPTY_RANGE_IN_CHAR_CLASS = -203;
    ONIGERR_MISMATCH_CODE_LENGTH_IN_CLASS_RANGE = -204;
    ONIGERR_TOO_MANY_MULTI_BYTE_RANGES = -205;
    ONIGERR_TOO_SHORT_MULTI_BYTE_STRING = -206;
    ONIGERR_TOO_BIG_BACKREF_NUMBER = -207;
    ONIGERR_INVALID_BACKREF = -208;
    ONIGERR_NUMBERED_BACKREF_OR_CALL_NOT_ALLOWED = -209;
    ONIGERR_TOO_LONG_WIDE_CHAR_VALUE = -212;
    ONIGERR_EMPTY_GROUP_NAME = -214;
    ONIGERR_INVALID_GROUP_NAME = -215;
    ONIGERR_INVALID_CHAR_IN_GROUP_NAME = -216;
    ONIGERR_UNDEFINED_NAME_REFERENCE = -217;
    ONIGERR_UNDEFINED_GROUP_REFERENCE = -218;
    ONIGERR_MULTIPLEX_DEFINED_NAME = -219;
    ONIGERR_MULTIPLEX_DEFINITION_NAME_CALL = -220;
    ONIGERR_NEVER_ENDING_RECURSION = -221;
    ONIGERR_GROUP_NUMBER_OVER_FOR_CAPTURE_HISTORY = -222;
    ONIGERR_INVALID_CHAR_PROPERTY_NAME = -223;
    ONIGERR_INVALID_CODE_POINT_VALUE = -400;
    ONIGERR_INVALID_WIDE_CHAR_VALUE = -400;
    ONIGERR_TOO_BIG_WIDE_CHAR_VALUE = -401;
    ONIGERR_NOT_SUPPORTED_ENCODING_COMBINATION = -402;
    ONIGERR_INVALID_COMBINATION_OF_OPTIONS = -403;
    ONIGERR_OVER_THREAD_PASS_LIMIT_COUNT = -1001
);

// OnigRegexType->state
define_consts!(c_int,
    ONIG_STATE_NORMAL     = 0;
    ONIG_STATE_SEARCHING  = 1;
    ONIG_STATE_COMPILING  = -1;
    ONIG_STATE_MODIFY     = -2
);

// Capture tree traverse callback returns
define_consts!(c_int,
    ONIG_TRAVERSE_CALLBACK_AT_FIRST = 1;
    ONIG_TRAVERSE_CALLBACK_AT_LAST  = 2;
    ONIG_TRAVERSE_CALLBACK_AT_BOTH  = 1 | 2
);

// Meta character specifiers, see onig_set_meta_char()
define_consts!(c_uint,
    ONIG_META_CHAR_ESCAPE           = 0;
    ONIG_META_CHAR_ANYCHAR          = 1;
    ONIG_META_CHAR_ANYTIME          = 2;
    ONIG_META_CHAR_ZERO_OR_ONE_TIME = 3;
    ONIG_META_CHAR_ONE_OR_MORE_TIME = 4;
    ONIG_META_CHAR_ANYCHAR_ANYTIME  = 5;
    ONIG_INEFFECTIVE_META_CHAR      = 0
);

// Infinite distance
define_consts!(OnigDistance, ONIG_INFINITE_DISTANCE = !0);

// Case fold flags
define_consts!(OnigCaseFoldType,
    ONIGENC_MAX_COMP_CASE_FOLD_CODE_LEN   = 3;
    ONIGENC_GET_CASE_FOLD_CODES_MAX_NUM   = 13;
    ONIGENC_CASE_FOLD_TURKISH_AZERI       = 1 << 20;
    ONIGENC_CASE_FOLD_MIN                 = 1 << 30;
    INTERNAL_ONIGENC_CASE_FOLD_MULTI_CHAR = 1 << 30
);

// Work size, config parameters and constants
define_consts!(c_int,
    ONIGENC_CODE_TO_MBC_MAXLEN     = 7;
    ONIGENC_MBC_CASE_FOLD_MAXLEN   = 18;
    ONIG_NREGION                   = 10;
    ONIG_MAX_BACKREF_NUM           = 1000;
    ONIG_MAX_REPEAT_NUM            = 100000;
    ONIG_MAX_MULTI_BYTE_RANGES_NUM = 10000;
    ONIG_MAX_ERROR_MESSAGE_LEN     = 90;
    ONIG_MAX_CAPTURE_HISTORY_GROUP = 31;
    ONIG_REGION_NOTPOS             = -1;
    ONIG_CHAR_TABLE_SIZE           = 256
);
