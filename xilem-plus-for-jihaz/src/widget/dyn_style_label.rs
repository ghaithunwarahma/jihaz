// Copyright 2019 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

#![warn(missing_docs)]

//! A label widget.
use std::mem::Discriminant;

// --- MARK: Modified ---
use accesskit::{Node, Role};
use jihaz_macros::debug_panic;
use masonry::parley::layout::Alignment;
use masonry::parley::{Layout, LayoutAccessibility};
use smallvec::SmallVec;
use tracing::{Span, trace_span};
use masonry::vello::Scene;
use masonry::vello::kurbo::{Affine, Size};
use masonry::vello::peniko::{BlendMode, Brush};

use masonry::core::{
    AccessCtx, AccessEvent, ArcStr, BoxConstraints, BrushIndex, EventCtx, LayoutCtx, PaintCtx,
    PointerEvent, PropertiesMut, PropertiesRef, QueryCtx, RegisterCtx,
    TextEvent, Update, UpdateCtx, Widget, WidgetId, WidgetMut, render_text,
};
use masonry::theme;
// --- MARK: Modified ---
use xilem::LineBreaking;

use crate::text::{CowStyleProperty, GeneralTextStyles}; 

/// Added padding between each horizontal edge of the widget
/// and the text in logical pixels.
const LABEL_X_PADDING: f64 = 2.0;

// --- MARK: Modified ---

/// A widget displaying non-interactive text.
///
/// This is useful for creating interactive widgets which internally
/// need support for displaying text, such as a button.
///
#[doc = masonry::include_screenshot!("widget/screenshots/masonry__widget__label__tests__styled_label.png", "Styled label.")]
pub struct Label {
    text_layout: Layout<BrushIndex>,
    accessibility: LayoutAccessibility,

    text: ArcStr,
    // --- MARK: Modified ---
    styles: GeneralTextStyles<BrushIndex>,
    /// Whether `text` or `styles` has been updated since `text_layout` was created.
    ///
    /// If they have, the layout needs to be recreated.
    styles_changed: bool,

    line_break_mode: LineBreaking,
    alignment: Alignment,
    /// Whether the alignment has changed since the last layout, which would force a re-alignment.
    alignment_changed: bool,
    /// The value of `max_advance` when this layout was last calculated.
    ///
    /// If it has changed, we need to re-perform line-breaking.
    last_max_advance: Option<f32>,

    /// The brush for drawing this label's text.
    ///
    /// Requires a new paint if edited whilst `disabled_brush` is not being used.
    brush: Brush,
    /// The brush to use whilst this widget is disabled.
    ///
    /// When this is `None`, `brush` will be used.
    /// Requires a new paint if edited whilst this widget is disabled.
    disabled_brush: Option<Brush>,
    /// Whether to hint whilst drawing the text.
    ///
    /// Should be disabled whilst an animation involving this label is ongoing.
    // TODO: What classes of animations?
    hint: bool,
}

// --- MARK: BUILDERS ---
impl Label {
    // --- MARK: Modified ---
    /// Create a new label with the given text.
    ///
    // This is written out fully to appease rust-analyzer; StyleProperty is imported but not recognised.
    /// To change the font size, use `with_style`, setting [`StyleProperty::FontSize`](parley::StyleProperty::FontSize).
    pub fn new(
        text: impl Into<ArcStr>, 
        styles: GeneralTextStyles<BrushIndex>
    ) -> Self {
        Self {
            text_layout: Layout::new(),
            accessibility: LayoutAccessibility::default(),
            text: text.into(),
            styles,
            styles_changed: true,
            line_break_mode: LineBreaking::Overflow,
            alignment: Alignment::Start,
            alignment_changed: true,
            last_max_advance: None,
            brush: theme::TEXT_COLOR.into(),
            disabled_brush: Some(theme::DISABLED_TEXT_COLOR.into()),
            hint: true,
        }
    }

    /// Get the current text of this label.
    ///
    /// To update the text of an active label, use [`set_text`](Self::set_text).
    pub fn text(&self) -> &ArcStr {
        &self.text
    }

    /// Set a style property for the new label.
    ///
    /// Setting [`StyleProperty::Brush`](parley::StyleProperty::Brush) is not supported.
    /// Use `with_brush` instead.
    ///
    /// To set a style property on an active label, use [`insert_style`](Self::insert_style).
    pub fn with_style(mut self, property: impl Into<CowStyleProperty<BrushIndex>>) -> Self {
        self.insert_style_inner(property.into());
        self
    }

    // --- MARK: Modified ---

    /// Set how line breaks will be handled by this label.
    ///
    /// To modify this on an active label, use [`set_line_break_mode`](Self::set_line_break_mode).
    pub fn with_line_break_mode(mut self, line_break_mode: LineBreaking) -> Self {
        self.line_break_mode = line_break_mode;
        self
    }

    /// Set the alignment of the text.
    ///
    /// Text alignment might have unexpected results when the label has no horizontal constraints.
    /// To modify this on an active label, use [`set_alignment`](Self::set_alignment).
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set the brush used to paint this label.
    ///
    /// In most cases, this will be the text's color, but gradients and images are also supported.
    ///
    /// To modify this on an active label, use [`set_brush`](Self::set_brush).
    #[doc(alias = "with_color")]
    pub fn with_brush(mut self, brush: impl Into<Brush>) -> Self {
        self.brush = brush.into();
        self
    }

    /// Set the brush which will be used to paint this label whilst it is disabled.
    ///
    /// If this is `None`, the [normal brush](Self::with_brush) will be used.
    /// To modify this on an active label, use [`set_disabled_brush`](Self::set_disabled_brush).
    #[doc(alias = "with_color")]
    pub fn with_disabled_brush(mut self, disabled_brush: impl Into<Option<Brush>>) -> Self {
        self.disabled_brush = disabled_brush.into();
        self
    }

    /// Set whether [hinting](https://en.wikipedia.org/wiki/Font_hinting) will be used for this label.
    ///
    /// Hinting is a process where text is drawn "snapped" to pixel boundaries to improve fidelity.
    /// The default is true, i.e. hinting is enabled by default.
    ///
    /// This should be set to false if the label will be animated at creation.
    /// The kinds of relevant animations include changing variable font parameters,
    /// translating or scaling.
    /// Failing to do so will likely lead to an unpleasant shimmering effect, as different parts of the
    /// text "snap" at different times.
    ///
    /// To modify this on an active label, use [`set_hint`](Self::set_hint).
    // TODO: Should we tell each widget if smooth scrolling is ongoing so they can disable their hinting?
    // Alternatively, we should automate disabling hinting at the Vello layer when composing.
    pub fn with_hint(mut self, hint: bool) -> Self {
        self.hint = hint;
        self
    }

    // --- MARK: Modified ---
    /// Shared logic between `with_style` and `insert_style`
    fn insert_style_inner(&mut self, property: CowStyleProperty<BrushIndex>) -> Option<CowStyleProperty<BrushIndex>> {
        if let CowStyleProperty::Brush(idx @ BrushIndex(1..))
        | CowStyleProperty::UnderlineBrush(Some(idx @ BrushIndex(1..)))
        | CowStyleProperty::StrikethroughBrush(Some(idx @ BrushIndex(1..))) = &property
        {
            debug_panic!(
                "Can't set a non-zero brush index ({idx:?}) on a `Label`, as it only supports global styling."
            );
        }
        self.styles.insert(property)
    }
}

// --- MARK: WIDGETMUT ---
impl Label {
    // --- MARK: Modified ---

    /// Overwritting the general style properties for the active label.
    pub fn overwrite_styles(
        this: &mut WidgetMut<'_, Self>,
        styles: GeneralTextStyles<BrushIndex>,
    ) -> GeneralTextStyles<BrushIndex> {
        let old = std::mem::replace(
            &mut this.widget.styles, 
            styles
        );
        this.widget.styles_changed = true;
        this.ctx.request_layout();
        old
    }

    // --- MARK: Modified ---

    // Note: These docs are lazy, but also have a decreased likelihood of going out of date.
    /// The runtime requivalent of [`with_style`](Self::with_style).
    ///
    /// Setting [`CowStyleProperty::Brush`](parley::CowStyleProperty::Brush) is not supported.
    /// Use [`set_brush`](Self::set_brush) instead.
    pub fn insert_style(
        this: &mut WidgetMut<'_, Self>,
        property: impl Into<CowStyleProperty<BrushIndex>>,
    ) -> Option<CowStyleProperty<BrushIndex>> {
        let old = this.widget.insert_style_inner(property.into());

        this.widget.styles_changed = true;
        this.ctx.request_layout();
        old
    }

    /// Keep only the styles for which `f` returns true.
    ///
    /// Styles which are removed return to Parley's default values.
    /// In most cases, these are the defaults for this widget.
    ///
    /// Of note, behaviour is unspecified for unsetting the [`FontSize`](parley::CowStyleProperty::FontSize).
    pub fn retain_styles(this: &mut WidgetMut<'_, Self>, f: impl FnMut(&CowStyleProperty<BrushIndex>) -> bool) {
        this.widget.styles.retain(f);

        this.widget.styles_changed = true;
        this.ctx.request_layout();
    }

    /// Remove the style with the discriminant `property`.
    ///
    /// To get the discriminant requires constructing a valid `CowStyleProperty` for the
    /// the desired property and passing it to [`core::mem::discriminant`].
    /// Getting this discriminant is usually possible in a `const` context.
    ///
    /// Styles which are removed return to Parley's default values.
    /// In most cases, these are the defaults for this widget.
    ///
    /// Of note, behaviour is unspecified for unsetting the [`FontSize`](parley::CowStyleProperty::FontSize).
    pub fn remove_style(
        this: &mut WidgetMut<'_, Self>,
        property: Discriminant<CowStyleProperty<BrushIndex>>,
    ) -> Option<CowStyleProperty<BrushIndex>> {
        let old = this.widget.styles.remove(property);

        this.widget.styles_changed = true;
        this.ctx.request_layout();
        old
    }

    // --- MARK: Modified ---

    /// Replace the text of this widget.
    pub fn set_text(this: &mut WidgetMut<'_, Self>, new_text: impl Into<ArcStr>) {
        this.widget.text = new_text.into();

        this.widget.styles_changed = true;
        this.ctx.request_layout();
    }

    /// The runtime requivalent of [`with_line_break_mode`](Self::with_line_break_mode).
    pub fn set_line_break_mode(this: &mut WidgetMut<'_, Self>, line_break_mode: LineBreaking) {
        this.widget.line_break_mode = line_break_mode;
        // We don't need to set an internal invalidation, as `max_advance` is always recalculated
        this.ctx.request_layout();
    }

    /// The runtime requivalent of [`with_alignment`](Self::with_alignment).
    pub fn set_alignment(this: &mut WidgetMut<'_, Self>, alignment: Alignment) {
        this.widget.alignment = alignment;

        this.widget.alignment_changed = true;
        this.ctx.request_layout();
    }

    #[doc(alias = "set_color")]
    /// The runtime requivalent of [`with_brush`](Self::with_brush).
    pub fn set_brush(this: &mut WidgetMut<'_, Self>, brush: impl Into<Brush>) {
        let brush = brush.into();
        this.widget.brush = brush;

        // We need to repaint unless the disabled brush is currently being used.
        if this.widget.disabled_brush.is_none() || this.ctx.is_disabled() {
            this.ctx.request_paint_only();
        }
    }

    /// The runtime requivalent of [`with_disabled_brush`](Self::with_disabled_brush).
    pub fn set_disabled_brush(this: &mut WidgetMut<'_, Self>, brush: impl Into<Option<Brush>>) {
        let brush = brush.into();
        this.widget.disabled_brush = brush;

        if this.ctx.is_disabled() {
            this.ctx.request_paint_only();
        }
    }

    /// The runtime requivalent of [`with_hint`](Self::with_hint).
    pub fn set_hint(this: &mut WidgetMut<'_, Self>, hint: bool) {
        this.widget.hint = hint;
        this.ctx.request_paint_only();
    }
}

// --- MARK: IMPL WIDGET ---
impl Widget for Label {
    fn on_pointer_event(
        &mut self,
        _ctx: &mut EventCtx,
        _props: &mut PropertiesMut<'_>,
        _event: &PointerEvent,
    ) {
    }

    fn accepts_pointer_interaction(&self) -> bool {
        false
    }

    fn on_text_event(
        &mut self,
        _ctx: &mut EventCtx,
        _props: &mut PropertiesMut<'_>,
        _event: &TextEvent,
    ) {
    }

    fn on_access_event(
        &mut self,
        _ctx: &mut EventCtx,
        _props: &mut PropertiesMut<'_>,
        _event: &AccessEvent,
    ) {
    }

    fn register_children(&mut self, _ctx: &mut RegisterCtx) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _props: &mut PropertiesMut<'_>, event: &Update) {
        match event {
            Update::DisabledChanged(_) => {
                if self.disabled_brush.is_some() {
                    ctx.request_paint_only();
                }
            }
            _ => {}
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        _props: &mut PropertiesMut<'_>,
        bc: &BoxConstraints,
    ) -> Size {
        let available_width = if bc.max().width.is_finite() {
            Some(bc.max().width as f32 - 2. * LABEL_X_PADDING as f32)
        } else {
            None
        };

        let max_advance = if self.line_break_mode == LineBreaking::WordWrap {
            available_width
        } else {
            None
        };
        let styles_changed = self.styles_changed;
        if self.styles_changed {
            let (font_ctx, layout_ctx) = ctx.text_contexts();
            // TODO: Should we use a different scale?
            let mut builder = layout_ctx.ranged_builder(font_ctx, &self.text, 1.0);
            
            for prop in self.styles.iterate_styles() {
                builder.push_default(prop.to_owned());
            }
            builder.build_into(&mut self.text_layout, &self.text);
            self.styles_changed = false;
        }

        if max_advance != self.last_max_advance || styles_changed {
            self.text_layout.break_all_lines(max_advance);
            self.last_max_advance = max_advance;
            self.alignment_changed = true;
        }

        // --- MARK: Modified ---
        // When Label is forced to occupy all available space,
        // and text is right to left, we'll have to provide the full available
        // width to the align function, so that it properly aligns the rtl text to the start.
        let special_rtl_case = bc.min().width != 0.0 && self.text_layout.is_rtl();

        let alignment_width = if self.alignment == Alignment::Start && !special_rtl_case {
            self.text_layout.width()
        } else if let Some(width) = available_width {
            // We use the full available space to calculate text alignment and therefore
            // determine the widget's current width.
            //
            // As a special case, we don't do that if the alignment is to the start.
            // In theory, we should be passed down how our parent expects us to be aligned;
            // however that isn't currently handled.
            //
            // This does effectively mean that the widget takes up all the available space and
            // therefore doesn't play nicely with adjacent widgets unless `Start` alignment is used.
            //
            // The coherent way to have multiple items laid out on the same line and alignment is for them to
            // be inside the same text layout object "region".
            width
        } else {
            // TODO: Warn on the rising edge of entering this state for this widget?
            self.text_layout.width()
        };
        if self.alignment_changed {
            self.text_layout
                .align(Some(alignment_width), self.alignment, false);
        }
        let text_size = Size::new(alignment_width.into(), self.text_layout.height().into());

        let label_size = Size {
            height: text_size.height,
            width: text_size.width + 2. * LABEL_X_PADDING,
        };
        bc.constrain(label_size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _props: &PropertiesRef<'_>, scene: &mut Scene) {
        if self.line_break_mode == LineBreaking::Clip {
            let clip_rect = ctx.size().to_rect();
            scene.push_layer(BlendMode::default(), 1., Affine::IDENTITY, &clip_rect);
        }
        let transform = Affine::translate((LABEL_X_PADDING, 0.));

        let brush = if ctx.is_disabled() {
            self.disabled_brush
                .clone()
                .unwrap_or_else(|| self.brush.clone())
        } else {
            self.brush.clone()
        };
        render_text(scene, transform, &self.text_layout, &[brush], self.hint);

        if self.line_break_mode == LineBreaking::Clip {
            scene.pop_layer();
        }
    }

    fn accessibility_role(&self) -> Role {
        Role::Label
    }

    // --- MARK: Modified ---
    fn accessibility(&mut self, _ctx: &mut AccessCtx, _props: &PropertiesRef<'_>, _node: &mut Node) {
        // let window_origin = ctx.window_origin();
        // self.accessibility.build_nodes(
        //     self.text.as_ref(),
        //     &self.text_layout,
        //     ctx,
        //     node,
        //     || NodeId::from(WidgetId::next()),
        //     window_origin.x + LABEL_X_PADDING,
        //     window_origin.y,
        // );
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        SmallVec::new()
    }

    fn make_trace_span(&self, ctx: &QueryCtx<'_>) -> Span {
        trace_span!("Label", id = ctx.widget_id().trace())
    }

    fn get_debug_text(&self) -> Option<String> {
        Some(self.text.to_string())
    }
}

// --- MARK: TESTS ---
// --- MARK: Modified ---
#[cfg(test)]
mod tests {
    
    use insta::assert_debug_snapshot;
    use masonry::parley::style::GenericFamily;
    
    use std::sync::Arc;
    use crate::text::{CowFontFamily, CowStyleProperty};

    use super::*;
    use masonry::assert_render_snapshot;
    use masonry::testing::TestHarness;
    use masonry::theme::{PRIMARY_DARK, PRIMARY_LIGHT};
    use masonry::widgets::{CrossAxisAlignment, Flex, SizedBox};

    #[test]
    fn simple_label() {
        
        let mut styles = GeneralTextStyles::empty();
        styles.insert(CowStyleProperty::FontStack(CowFontFamily::Named(
            Arc::from("system-ui")
        ).into()));
        
        let label = Label::new("Hello", styles);

        let mut harness = TestHarness::create(label);

        assert_debug_snapshot!(harness.root_widget());
        assert_render_snapshot!(harness, "hello");
    }

    #[test]
    fn styled_label() {

        let mut styles = GeneralTextStyles::empty();
        styles.insert(CowFontFamily::Generic(GenericFamily::Monospace).into());
        styles.insert(CowStyleProperty::FontSize(20.0));

        let label = Label::new(
            "The quick brown fox jumps over the lazy dog", 
            styles
        )
            .with_brush(PRIMARY_LIGHT)
            .with_line_break_mode(LineBreaking::WordWrap)
            .with_alignment(Alignment::Middle);

        let mut harness = TestHarness::create_with_size(label, Size::new(200.0, 200.0));

        assert_render_snapshot!(harness, "styled_label");
    }

    #[test]
    fn underline_label() {
        let mut styles = GeneralTextStyles::empty();
        styles.insert(CowStyleProperty::Underline(true));

        let label = Label::new("Emphasis", styles)
            .with_line_break_mode(LineBreaking::WordWrap);
        let mut harness = TestHarness::create_with_size(label, Size::new(100.0, 20.));

        assert_render_snapshot!(harness, "underline_label");
    }
    #[test]
    fn strikethrough_label() {

        let mut styles = GeneralTextStyles::empty();
        styles.insert(CowStyleProperty::Strikethrough(true));
        styles.insert(CowStyleProperty::StrikethroughSize(Some(4.)));

        let label = Label::new("Tpyo", styles)
            .with_line_break_mode(LineBreaking::WordWrap);

        let mut harness = TestHarness::create_with_size(label, Size::new(100.0, 20.));

        assert_render_snapshot!(harness, "strikethrough_label");
    }

    #[test]
    /// A wrapping label's alignment should be respected, regardkess of
    /// its parent's alignment.
    fn label_alignment_flex() {
        fn base_label() -> Label {
            
            let mut styles = GeneralTextStyles::empty();
            styles.insert(CowStyleProperty::FontSize(10.0));
            
            Label::new("Hello", styles)
                .with_line_break_mode(LineBreaking::WordWrap)
        }
        let label1 = base_label().with_alignment(Alignment::Start);
        let label2 = base_label().with_alignment(Alignment::Middle);
        let label3 = base_label().with_alignment(Alignment::End);
        let label4 = base_label().with_alignment(Alignment::Start);
        let label5 = base_label().with_alignment(Alignment::Middle);
        let label6 = base_label().with_alignment(Alignment::End);
        let flex = Flex::column()
            .with_flex_child(label1, CrossAxisAlignment::Start)
            .with_flex_child(label2, CrossAxisAlignment::Start)
            .with_flex_child(label3, CrossAxisAlignment::Start)
            // Text alignment start is "overwritten" by CrossAxisAlignment::Center.
            .with_flex_child(label4, CrossAxisAlignment::Center)
            .with_flex_child(label5, CrossAxisAlignment::Center)
            .with_flex_child(label6, CrossAxisAlignment::Center)
            .gap(0.0);

        let mut harness = TestHarness::create_with_size(flex, Size::new(80.0, 80.0));

        assert_render_snapshot!(harness, "label_alignment_flex");
    }

    #[test]
    fn line_break_modes() {
        let widget = Flex::column()
            .with_flex_spacer(1.0)
            .with_child(
                SizedBox::new(
                    Label::new("The quick brown fox jumps over the lazy dog", GeneralTextStyles::empty())
                        .with_line_break_mode(LineBreaking::WordWrap),
                )
                .width(200.0),
            )
            .with_spacer(20.0)
            .with_child(
                SizedBox::new(
                    Label::new("The quick brown fox jumps over the lazy dog", GeneralTextStyles::empty())
                        .with_line_break_mode(LineBreaking::Clip),
                )
                .width(200.0),
            )
            .with_spacer(20.0)
            .with_child(
                SizedBox::new(
                    Label::new("The quick brown fox jumps over the lazy dog", GeneralTextStyles::empty())
                        .with_line_break_mode(LineBreaking::Overflow),
                )
                .width(200.0),
            )
            .with_flex_spacer(1.0);

        let mut harness = TestHarness::create(widget);

        assert_render_snapshot!(harness, "line_break_modes");
    }

    #[test]
    fn edit_label() {
        
        let mut styles = GeneralTextStyles::empty();
        styles.insert(CowFontFamily::Generic(GenericFamily::Monospace).into());
        styles.insert(CowStyleProperty::FontSize(20.0));

        let image_1 = {
            let label = Label::new("The quick brown fox jumps over the lazy dog", styles)
                .with_brush(PRIMARY_LIGHT)
                .with_line_break_mode(LineBreaking::WordWrap)
                .with_alignment(Alignment::Middle);

            let mut harness = TestHarness::create_with_size(label, Size::new(50.0, 50.0));

            harness.render()
        };

        let mut styles_1= GeneralTextStyles::empty();
        styles_1.insert(CowStyleProperty::FontSize(40.0));

        let mut styles_2 = GeneralTextStyles::empty();
        styles_2.insert(CowFontFamily::Generic(GenericFamily::Monospace).into());
        styles_2.insert(CowStyleProperty::FontSize(20.0));

        let image_2 = {
            let label = Label::new("Hello world", styles_1)
                .with_brush(PRIMARY_DARK);

            let mut harness = TestHarness::create_with_size(label, Size::new(50.0, 50.0));

            harness.edit_root_widget(|mut label| {
                let mut label = label.downcast::<Label>();
                Label::set_text(&mut label, "The quick brown fox jumps over the lazy dog");
                Label::set_brush(&mut label, PRIMARY_LIGHT);
                Label::overwrite_styles(&mut label, styles_2);
                Label::set_line_break_mode(&mut label, LineBreaking::WordWrap);
                Label::set_alignment(&mut label, Alignment::Middle);
            });

            harness.render()
        };

        // We don't use assert_eq because we don't want rich assert
        assert!(image_1 == image_2);
    }
}
