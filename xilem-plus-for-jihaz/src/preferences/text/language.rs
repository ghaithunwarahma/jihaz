use serde::{Deserialize, Serialize};

// --- MARK: Language ---
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Language {
    Arabic,
    English,
}

impl Language {
    /// Confirm if the language is right to left
    pub fn is_rtl(self) -> bool {
        match self {
            Language::Arabic => true,
            Language::English => false,
        }
    }

    /// Get the language name in own language
    pub fn to_str(self) -> &'static str {
        self.to_str_in(self)
    }

    /// Get the language name is the given language
    pub fn to_str_in(self, in_language: Self) -> &'static str {
        match in_language {
            Language::Arabic => {
                match self {
                    Language::Arabic => "العربية",
                    Language::English => "الإنجليزية",
                }
            }
            Language::English => {
                match self {
                    Language::Arabic => "Arabic",
                    Language::English => "English",
                }
            }
        }
    }
}