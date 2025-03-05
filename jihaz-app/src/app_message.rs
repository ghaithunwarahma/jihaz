

#[derive(Default, Copy, Clone, Debug)]
pub enum AppMessageResult {
    /// Should generate packages
    GeneratePackages,
    /// The event handler discarded the event.
    ///
    /// This is the variant that you **almost always want** when you're not returning
    /// an action.
    #[allow(unused)]
    #[default]
    Nop,
}