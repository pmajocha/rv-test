use std::ffi::CString;

mod c {
    extern "C" {
        /// Removes `pattern` from memory
        pub fn validate(pattern: *const libc::c_char) -> bool;
        /// Removes `pattern` from memory
        pub fn error_code(pattern: *const libc::c_char) -> i32;
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
        Some(RegexError::StringTrailingCharactersInTheMiddle)
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum RegexError {
    // re2 errors
    ErrorInternal = 1,
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
    StringTrailingCharactersInTheMiddle,
}

impl TryFrom<i32> for RegexError {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ErrorInternal),
            2 => Ok(Self::ErrorBadEscape),          
            3 => Ok(Self::ErrorBadCharClass),       
            4 => Ok(Self::ErrorBadCharRange),       
            5 => Ok(Self::ErrorMissingBracket),     
            6 => Ok(Self::ErrorMissingParen),       
            7 => Ok(Self::ErrorUnexpectedParen),    
            8 => Ok(Self::ErrorTrailingBackslash),  
            9 => Ok(Self::ErrorRepeatArgument),     
            10 => Ok(Self::ErrorRepeatSize),        
            11 => Ok(Self::ErrorRepeatOp),          
            12 => Ok(Self::ErrorBadPerlOp),         
            13 => Ok(Self::ErrorBadUTF8),            
            14 => Ok(Self::ErrorBadNamedCapture),    
            15 => Ok(Self::ErrorPatternTooLarge), 
            _ => Err(())
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
            RegexError::StringTrailingCharactersInTheMiddle => write!(f, "pattern had string terminating characters '/0' in the string")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_check_validation() {
        assert!(is_valid(".*"));
        assert!(!is_valid("((("));
    }

    #[test]
    fn it_return_proper_error_code() {
        assert_eq!(check_error("("), Some(RegexError::ErrorMissingParen));
    }
}
