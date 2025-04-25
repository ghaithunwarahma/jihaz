
use masonry::core::BrushIndex;
// using peniko directly to enable serde feature
// use peniko::Brush;
use serde::{Deserialize, Serialize};
use super::{GeneralTextStyles, SdStyleProperty};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SdGeneralTextStyles(Vec<SdStyleProperty>);

impl SdGeneralTextStyles {
    pub fn default_styles() -> Self {
        Self::from(&GeneralTextStyles::default_styles())
    }
}

impl From<&GeneralTextStyles<BrushIndex>> for SdGeneralTextStyles {
    fn from(value: &GeneralTextStyles<BrushIndex>) -> Self {
        Self(
            value.inner()
                .values()
                .map(SdStyleProperty::from)
                .collect()
        )
    }
}

impl From<SdGeneralTextStyles> for GeneralTextStyles<BrushIndex> {
    fn from(value: SdGeneralTextStyles) -> Self {
        let mut general_styles = GeneralTextStyles::empty();
        value.0.into_iter().for_each(|s| {
            general_styles.insert(s.into());
        });
        general_styles
    }
}
