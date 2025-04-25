pub mod color {
    use xilem::Color;
    // color-picked from: 
    pub const SAND_ORANGE: Color = Color::from_rgb8(149, 61, 22);
    /// color-picked from SAND_ORANGE with 70% opacity over a black background
    pub const SAND_ORANGE_DARKER: Color = Color::from_rgb8(101, 43, 16);
    
    /// Original, color-picked from: https://www.pexels.com/photo/man-sitting-by-the-fire-on-a-desert-16386939/
    pub const SAND_BROWN: Color = Color::from_rgb8(50, 38, 37);
    /// color-picked from SAND_ORANGE with 80.9% opacity over a black background
    pub const SAND_BROWN_DARK: Color = Color::from_rgb8(41, 31, 30);
    /// color-picked from SAND_ORANGE with 61.8% opacity over a black background
    pub const SAND_BROWN_DARKER: Color = Color::from_rgb8(31, 23, 23);
    /// color-picked from SAND_ORANGE with 80.9% opacity over a white background
    pub const SAND_BROWN_LIGHT: Color = Color::from_rgb8(90, 80, 79);
    /// color-picked from SAND_ORANGE with 61.8% opacity over a white background
    pub const SAND_BROWN_LIGHTER: Color = Color::from_rgb8(129, 120, 120);

    // pub const BACKGROUND_70: Color = SAND_BROWN.with_alpha(0.7);
    // pub const BACKGROUND_80: Color = SAND_BROWN.with_alpha(0.8);
    // pub const BACKGROUND_90: Color = SAND_BROWN.with_alpha(0.9);

    pub mod default_light {
        use super::*;
        /// The normal background color
        pub const BACKGROUND: Color = SAND_BROWN_LIGHT;
        /// Normally for hovered widgets or forground appearing widgets
        pub const BACKGROUND_2X: Color = SAND_BROWN;
        /// Normally for clicked widgets or hovered forground appearing widgets
        pub const BACKGROUND_3X: Color = SAND_BROWN_DARK;
        /// Normally for clicked forground appearing widgets
        pub const BACKGROUND_4X: Color = SAND_BROWN_DARKER;

        pub const HOVERED: Color = BACKGROUND_2X;
        pub const CLICKED: Color = BACKGROUND_3X;

        pub const BORDER: Color = BACKGROUND_3X;
        pub const BORDER_HOVERED: Color = BACKGROUND_4X;
    }
    pub mod default_dark {
        use super::*;
        /// The normal background color
        pub const BACKGROUND: Color = SAND_BROWN_DARK;
        /// Normally for hovered widgets or forground appearing widgets
        pub const BACKGROUND_2X: Color = SAND_BROWN;
        /// Normally for clicked widgets or hovered forground appearing widgets
        pub const BACKGROUND_3X: Color = SAND_BROWN_LIGHT;
        /// Normally for clicked forground appearing widgets
        pub const BACKGROUND_4X: Color = SAND_BROWN_LIGHTER;
    
        pub const ITEM: Color = BACKGROUND_2X;
        pub const HOVERED: Color = BACKGROUND_3X;
        pub const CLICKED: Color = BACKGROUND_4X;

        pub const BORDER: Color = BACKGROUND_3X;
        pub const BORDER_HOVERED: Color = BACKGROUND_4X;
    }

}

pub mod distance {
    pub const BUTTON_BORDER_RADIUS: f64 = 5.;
    pub const BUTTON_BORDER_WIDTH: f64 = 2.;
}

