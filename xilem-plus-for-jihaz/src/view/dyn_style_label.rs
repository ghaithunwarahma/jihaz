// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

// --- MARK: Modified ---
use masonry::core::{ArcStr, BrushIndex};
use masonry::vello::peniko::Brush;

use xilem::core::{DynMessage, Mut, ViewMarker};
use xilem::{Color, LineBreaking, Pod, TextAlignment, ViewCtx};
use xilem_core::{MessageResult, View, ViewId};
use crate::{text::GeneralTextStyles, widget};

/// A non-interactive text element.
/// # Example
///
/// ```ignore
/// use xilem::palette;
/// use xilem::view::label;
/// use masonry::TextAlignment;
/// use masonry::parley::fontique;
///
/// label("Text example.")
///     .brush(palette::css::RED)
///     .alignment(TextAlignment::Middle)
///     .text_size(24.0)
///     .weight(FontWeight::BOLD)
///     .with_font(fontique::GenericFamily::Serif)
/// ```
pub fn label(
    label: impl Into<ArcStr>,
    general_styles: GeneralTextStyles<BrushIndex>,
) -> Label {
    Label {
        label: label.into(),
        general_styles,
        text_brush: Color::WHITE.into(),
        alignment: TextAlignment::default(),
        line_break_mode: LineBreaking::Overflow,
    }
}

/// The [`View`] created by [`label`] from a text which `impl Into<`[`ArcStr`]`>`.
///
/// See `label` documentation for more context.
#[must_use = "View values do nothing unless provided to Xilem."]
pub struct Label {
    pub label: ArcStr,
    general_styles: GeneralTextStyles<BrushIndex>,
    text_brush: Brush,
    alignment: TextAlignment,
    line_break_mode: LineBreaking, // TODO: add more attributes of `masonry::widget::Label`
}

impl Label {
    /// In most cases brush sets text color, but gradients and images are also supported.
    #[doc(alias = "color")]
    pub fn brush(mut self, brush: impl Into<Brush>) -> Self {
        self.text_brush = brush.into();
        self
    }

    /// Sets text alignment: `Start`, `Middle`, `End` or `Justified`.
    pub fn alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    // --- MARK: Modified ---

    /// Set how line breaks will be handled by this label (i.e. if there is insufficient horizontal space).
    pub fn line_break_mode(mut self, line_break_mode: LineBreaking) -> Self {
        self.line_break_mode = line_break_mode;
        self
    }
}

// --- MARK: Modified ---
// impl<T> From<T> for Label
// where
//     T: Into<ArcStr>,
// {
//     fn from(text: T) -> Self {
//         label(text)
//     }
// }

impl ViewMarker for Label {}
impl<State, Action> View<State, Action, ViewCtx> for Label {
    type Element = Pod<widget::Label>;
    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        // --- MARK: Modified ---
        let widget_pod = ctx.new_pod(
            widget::Label::new(self.label.clone(), self.general_styles.clone())
                .with_brush(self.text_brush.clone())
                .with_alignment(self.alignment)
                .with_line_break_mode(self.line_break_mode),
        );
        (widget_pod, ())
    }

    fn rebuild(
        &self,
        prev: &Self,
        (): &mut Self::ViewState,
        _ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        if prev.label != self.label {
            widget::Label::set_text(&mut element, self.label.clone());
        }
        if prev.text_brush != self.text_brush {
            widget::Label::set_brush(&mut element, self.text_brush.clone());
        }
        if prev.alignment != self.alignment {
            widget::Label::set_alignment(&mut element, self.alignment);
        }
        // --- MARK: Modified ---
        if &prev.general_styles != &self.general_styles {
            widget::Label::overwrite_styles(&mut element, self.general_styles.clone());
        }
        if prev.line_break_mode != self.line_break_mode {
            widget::Label::set_line_break_mode(&mut element, self.line_break_mode);
        }
    }

    fn teardown(&self, (): &mut Self::ViewState, _: &mut ViewCtx, _: Mut<Self::Element>) {}

    fn message(
        &self,
        (): &mut Self::ViewState,
        _id_path: &[ViewId],
        message: DynMessage,
        _app_state: &mut State,
    ) -> MessageResult<Action> {
        tracing::error!(
            "Message arrived in Label::message, but Label doesn't consume any messages, this is a bug"
        );
        MessageResult::Stale(message)
    }
}
