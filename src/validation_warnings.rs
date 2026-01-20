/// Optional validation warnings for TwiML best practices
///
/// These are NOT errors - the generated TwiML is valid.
/// These warnings help developers avoid common pitfalls.
use crate::messaging::{MessagingResponse, MessagingVerb};

/// Warning types for TwiML best practices
#[derive(Debug, Clone, PartialEq)]
pub enum TwiMLWarning {
    /// A Redirect verb appears after a Message with an action attribute
    /// The Redirect will be unreachable because control flow will transfer to the action URL
    UnreachableRedirectAfterMessageWithAction {
        message_index: usize,
        redirect_index: usize,
    },

    /// An empty Redirect URL will create an infinite loop
    EmptyRedirectUrl { redirect_index: usize },

    /// Multiple verbs after a Redirect (they will never be reached)
    UnreachableVerbsAfterRedirect {
        redirect_index: usize,
        unreachable_count: usize,
    },
}

impl std::fmt::Display for TwiMLWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwiMLWarning::UnreachableRedirectAfterMessageWithAction {
                message_index,
                redirect_index,
            } => {
                write!(
                    f,
                    "Warning: Redirect at index {} may be unreachable because Message at index {} has an action attribute",
                    redirect_index, message_index
                )
            }
            TwiMLWarning::EmptyRedirectUrl { redirect_index } => {
                write!(
                    f,
                    "Warning: Redirect at index {} has an empty URL, which will create an infinite loop",
                    redirect_index
                )
            }
            TwiMLWarning::UnreachableVerbsAfterRedirect {
                redirect_index,
                unreachable_count,
            } => {
                write!(
                    f,
                    "Warning: {} verb(s) after Redirect at index {} will never be reached",
                    unreachable_count, redirect_index
                )
            }
        }
    }
}

impl MessagingResponse {
    /// Validate the TwiML response and return warnings (if any)
    ///
    /// This does NOT affect XML generation - the TwiML is still valid.
    /// Warnings help identify potential logic issues.
    ///
    /// # Example
    /// ```
    /// use twiml_rust::messaging::MessagingResponse;
    ///
    /// let response = MessagingResponse::new()
    ///     .message("Hello!")
    ///     .redirect("https://example.com");
    ///
    /// let warnings = response.validate();
    /// assert!(warnings.is_empty()); // No warnings - this is fine
    /// ```
    pub fn validate(&self) -> Vec<TwiMLWarning> {
        let mut warnings = Vec::new();

        // Check for unreachable verbs after Redirect
        for (i, verb) in self.verbs.iter().enumerate() {
            if matches!(verb, MessagingVerb::Redirect(_)) {
                let remaining = self.verbs.len() - i - 1;
                if remaining > 0 {
                    warnings.push(TwiMLWarning::UnreachableVerbsAfterRedirect {
                        redirect_index: i,
                        unreachable_count: remaining,
                    });
                }
            }
        }

        warnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messaging::MessagingResponse;

    #[test]
    fn test_no_warnings_for_simple_message() {
        let response = MessagingResponse::new().message("Hello!");
        let warnings = response.validate();
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_no_warnings_for_message_then_redirect() {
        let response = MessagingResponse::new()
            .message("Hello!")
            .redirect("https://example.com");
        let warnings = response.validate();
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_warning_for_verbs_after_redirect() {
        let response = MessagingResponse::new()
            .redirect("https://example.com")
            .message("This will never be sent");

        let warnings = response.validate();
        assert_eq!(warnings.len(), 1);
        assert!(matches!(
            warnings[0],
            TwiMLWarning::UnreachableVerbsAfterRedirect {
                redirect_index: 0,
                unreachable_count: 1
            }
        ));
    }

    #[test]
    fn test_warning_display() {
        let warning = TwiMLWarning::UnreachableVerbsAfterRedirect {
            redirect_index: 0,
            unreachable_count: 2,
        };
        let display = format!("{}", warning);
        assert!(display.contains("2 verb(s)"));
        assert!(display.contains("index 0"));
    }
}
