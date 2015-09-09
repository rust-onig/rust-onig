extern crate libc;

#[allow(non_camel_case_types)]
pub enum regex_t {}

pub enum OnigRegion {}

extern "C" {

    // int onig_init(void)
    pub fn onig_init() -> u32;

    // int onig_error_code_to_str(UChar* err_buf, int err_code, ...)

    // void onig_set_warn_func(OnigWarnFunc func)

    // void onig_set_verb_warn_func(OnigWarnFunc func)

    // void onig_free(regex_t* reg)
    pub fn onig_free(reg: *const regex_t);

    // void onig_free_body(regex_t* reg)
    pub fn onig_free_body(reg: *const regex_t);

    // OnigRegion* onig_region_new(void)
    pub fn onig_region_new() -> *const OnigRegion;

    // void onig_region_free(OnigRegion* region, int free_self)
    pub fn onig_region_free(region: *const OnigRegion, free_self: libc::c_int);

    // void onig_region_copy(OnigRegion* to, OnigRegion* from)
    pub fn onig_region_copy(to: *const OnigRegion, from: *const OnigRegion);

    // void onig_region_clear(OnigRegion* region)
    pub fn onig_region_clear(region: *const OnigRegion);

    // int onig_region_resize(OnigRegion* region, int n)
    pub fn onig_region_resize(
        region: *const OnigRegion, n: libc::c_int) -> libc::c_int;

    // int onig_number_of_names(regex_t* reg)
    pub fn onig_number_of_name(reg: *const regex_t);

    // OnigEncoding     onig_get_encoding(regex_t* reg)

    // OnigOptionType   onig_get_options(regex_t* reg)

    // OnigCaseFoldType onig_get_case_fold_flag(regex_t* reg)

    // OnigSyntaxType*  onig_get_syntax(regex_t* reg)

    // int onig_number_of_captures(regex_t* reg)

    // int onig_number_of_capture_histories(regex_t* reg)

    // OnigCaptureTreeNode* onig_get_capture_tree(OnigRegion* region)

    // int onig_noname_group_capture_is_active(regex_t* reg)

    // UChar* onigenc_get_prev_char_head(OnigEncoding enc, const UChar* start, const UChar* s)

    // int onigenc_strlen(OnigEncoding enc, const UChar* s, const UChar* end)

    // int onigenc_strlen_null(OnigEncoding enc, const UChar* s)

    // int onigenc_str_bytelen_null(OnigEncoding enc, const UChar* s)

    // int onig_set_default_syntax(OnigSyntaxType* syntax)

    // void onig_copy_syntax(OnigSyntaxType* to, OnigSyntaxType* from)

    // unsigned int onig_get_syntax_op(OnigSyntaxType* syntax)

    // unsigned int onig_get_syntax_op2(OnigSyntaxType* syntax)

    // unsigned int onig_get_syntax_behavior(OnigSyntaxType* syntax)

    // OnigOptionType onig_get_syntax_options(OnigSyntaxType* syntax)

    // void onig_set_syntax_op(OnigSyntaxType* syntax, unsigned int op)

    // void onig_set_syntax_op2(OnigSyntaxType* syntax, unsigned int op2)

    // void onig_set_syntax_behavior(OnigSyntaxType* syntax, unsigned int behavior)

    // void onig_set_syntax_options(OnigSyntaxType* syntax, OnigOptionType options)

    // void onig_copy_encoding(OnigEncoding to, OnigOnigEncoding from)

    // OnigCaseFoldType onig_get_default_case_fold_flag()

    // int onig_set_default_case_fold_flag(OnigCaseFoldType case_fold_flag)

    // unsigned int onig_get_match_stack_limit_size(void)

    // int onig_set_match_stack_limit_size(unsigned int size)

    // int onig_end(void)

    // const char* onig_version(void)
    pub fn onig_version() -> *const libc::c_char;
}
