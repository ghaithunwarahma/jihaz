//! Elements and tasks that help with composing text

pub mod buffer;
pub mod char_buffer;
pub mod chars;
pub mod command;
pub mod cursor;
pub mod editor;
pub mod encoding;
pub mod indent;
pub mod lens;
pub mod line_ending;
pub mod mode;
pub mod movement;
pub mod paragraph;
pub mod register;
pub mod selection;
pub mod soft_tab;
pub mod syntax_util;
pub mod util;
pub mod word;

pub use lapce_xi_rope as xi_rope;