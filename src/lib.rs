use std::ffi::CString;

mod c {
    #[repr(C)]
    pub struct MatchResult {
        pub error_code: i32,
        pub is_match: bool,
    }

    extern "C" {
        pub fn validate(pattern: *const libc::c_char) -> bool;
        pub fn error_code(pattern: *const libc::c_char) -> i32;
        pub fn is_match(pattern: *const libc::c_char, str: *const libc::c_char) -> MatchResult;
    }
}

pub fn is_valid(pattern: &str) -> bool {
    CString::new(pattern)
        // SAFETY: underlying implementation copies the string and deletes it on its own. We still have ownership of the pattern.
        .map(|c_str| unsafe { c::validate(c_str.as_ptr() as *const _) })
        .unwrap_or(false)    
}

pub fn check_error(pattern: &str) -> Option<RegexError> {
    if let Ok(c_str) = CString::new(pattern) {
        // SAFETY: underlying implementation copies the string and deletes it on its own. We still have ownership of the pattern.
        let code = unsafe { c::error_code(c_str.as_ptr() as *const _) };
        code.try_into().ok()
    } else {
        Some(RegexError::StringTerminatingCharactersInTheMiddle)
    }
}

pub fn matches(pattern: &str, text: &str) -> Result<bool, RegexError> {
    match (CString::new(pattern), CString::new(text)) {
        (Ok(pattern), Ok(text)) => {
            match unsafe { c::is_match(pattern.as_ptr() as *const _, text.as_ptr() as *const _) } {
                c::MatchResult { error_code: 0, is_match } => Ok(is_match),
                c::MatchResult { error_code, .. } => Err(RegexError::from(error_code)),
            }
        },
        _ => Err(RegexError::StringTerminatingCharactersInTheMiddle)
    }    
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum RegexError {
    // re2 errors
    ErrorInternal,
    ErrorBadEscape,          
    ErrorBadCharClass,       
    ErrorBadCharRange,
    ErrorMissingBracket,     
    ErrorMissingParen,       
    ErrorUnexpectedParen,    
    ErrorTrailingBackslash,
    ErrorRepeatArgument,
    ErrorRepeatSize,         
    ErrorRepeatOp,           
    ErrorBadPerlOp,          
    ErrorBadUTF8,
    ErrorBadNamedCapture,
    ErrorPatternTooLarge,
    // rust errors
    StringTerminatingCharactersInTheMiddle,
    ParseErrorCode(i32),
}

impl From<i32> for RegexError {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::ErrorInternal,
            2 => Self::ErrorBadEscape,          
            3 => Self::ErrorBadCharClass,       
            4 => Self::ErrorBadCharRange,       
            5 => Self::ErrorMissingBracket,     
            6 => Self::ErrorMissingParen,       
            7 => Self::ErrorUnexpectedParen,    
            8 => Self::ErrorTrailingBackslash,  
            9 => Self::ErrorRepeatArgument,     
            10 => Self::ErrorRepeatSize,        
            11 => Self::ErrorRepeatOp,          
            12 => Self::ErrorBadPerlOp,         
            13 => Self::ErrorBadUTF8,            
            14 => Self::ErrorBadNamedCapture,    
            15 => Self::ErrorPatternTooLarge, 
            _ => Self::ParseErrorCode(value),
        }
    }
}

impl std::error::Error for RegexError {}

impl std::fmt::Display for RegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexError::ErrorInternal => write!(f, "engine's internal error"),
            RegexError::ErrorBadEscape => write!(f, "bad escape sequence"),
            RegexError::ErrorBadCharClass => write!(f, "bad character class"),
            RegexError::ErrorBadCharRange => write!(f, "bad character class range"),
            RegexError::ErrorMissingBracket => write!(f, "missing closing ]"),
            RegexError::ErrorMissingParen => write!(f, "missing closing )"),
            RegexError::ErrorUnexpectedParen => write!(f, "unexpected closing )"),
            RegexError::ErrorTrailingBackslash => write!(f, "trailing \\ at end of regexp"),
            RegexError::ErrorRepeatArgument => write!(f, "repeat argument missing, e.g. \"*\""),
            RegexError::ErrorRepeatSize => write!(f, "bad repetition argument"),
            RegexError::ErrorRepeatOp => write!(f, "bad repetition operator"),
            RegexError::ErrorBadPerlOp => write!(f, "bad perl operator"),
            RegexError::ErrorBadUTF8 => write!(f, "invalid UTF-8 in regexp"),
            RegexError::ErrorBadNamedCapture => write!(f, "bad named capture group"),
            RegexError::ErrorPatternTooLarge => write!(f, "pattern too large"),
            RegexError::StringTerminatingCharactersInTheMiddle => write!(f, "pattern had string terminating characters '/0' in the string"),
            RegexError::ParseErrorCode(code) => write!(f, "error code is outside of range of known error codes: {code}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_validation() {
        assert!(is_valid(".*"));
        assert!(!is_valid("((("));
    }

    #[test]
    fn it_returns_proper_error_code() {
        assert_eq!(check_error("("), Some(RegexError::ErrorMissingParen));
    }

    #[test]
    fn it_matches_regex() {
        assert_eq!(matches("(", "abc"), Err(RegexError::ErrorMissingParen));
        assert_eq!(matches("[a-zA-Z]{4}", "xxz abc"), Ok(false));
        assert_eq!(matches("[a-zA-Z]{4}", "xxzA abc"), Ok(true));
    }
}
