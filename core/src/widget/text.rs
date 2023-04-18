//! Write some text for your users to read.
use crate::alignment;
use crate::layout;
use crate::renderer;
use crate::text;
use crate::widget::Tree;
use crate::{
    Color, Element, Layout, Length, Pixels, Point, Rectangle, Size, Widget,
};

use std::borrow::Cow;

/// A paragraph of text.
#[allow(missing_debug_implementations)]
pub struct Text<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    content: Cow<'a, str>,
    size: Option<f32>,
    width: Length,
    height: Length,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    font: Option<Renderer::Font>,
    style: <Renderer::Theme as StyleSheet>::Style,
    advanced_shape: bool,
}

impl<'a, Renderer> Text<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Create a new fragment of [`Text`] with the given contents.
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Text {
            content: content.into(),
            size: None,
            font: None,
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            style: Default::default(),
            advanced_shape: false,
        }
    }

    /// Sets the size of the [`Text`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().0);
        self
    }

    /// Sets the [`Font`] of the [`Text`].
    ///
    /// [`Font`]: crate::text::Renderer::Font
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the style of the [`Text`].
    pub fn style(
        mut self,
        style: impl Into<<Renderer::Theme as StyleSheet>::Style>,
    ) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the width of the [`Text`] boundaries.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Text`] boundaries.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`alignment::Horizontal`] of the [`Text`].
    pub fn horizontal_alignment(
        mut self,
        alignment: alignment::Horizontal,
    ) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the [`alignment::Vertical`] of the [`Text`].
    pub fn vertical_alignment(
        mut self,
        alignment: alignment::Vertical,
    ) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Enables advanced text shaping and font fallback for the [`Text`].
    ///
    /// You will need to enable this if the text contains a complex script, the
    /// font used needs it, and/or multiple fonts in your system may be needed
    /// to display all of the glyphs.
    ///
    /// If your text isn't displaying properly, try enabling this!
    ///
    /// Advanced shaping is expensive! You should only enable it when necessary.
    pub fn advanced_shape(mut self) -> Self {
        self.advanced_shape = true;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Text<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = self.size.unwrap_or_else(|| renderer.default_size());

        let bounds = limits.max();

        let (width, height) = renderer.measure(
            &self.content,
            size,
            self.font.unwrap_or_else(|| renderer.default_font()),
            bounds,
            self.advanced_shape,
        );

        let size = limits.resolve(Size::new(width, height));

        layout::Node::new(size)
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        draw(
            renderer,
            style,
            layout,
            &self.content,
            self.size,
            self.font,
            theme.appearance(self.style.clone()),
            self.horizontal_alignment,
            self.vertical_alignment,
            self.advanced_shape,
        );
    }
}

/// Draws text using the same logic as the [`Text`] widget.
///
/// Specifically:
///
/// * If no `size` is provided, the default text size of the `Renderer` will be
///   used.
/// * If no `color` is provided, the [`renderer::Style::text_color`] will be
///   used.
/// * The alignment attributes do not affect the position of the bounds of the
///   [`Layout`].
pub fn draw<Renderer>(
    renderer: &mut Renderer,
    style: &renderer::Style,
    layout: Layout<'_>,
    content: &str,
    size: Option<f32>,
    font: Option<Renderer::Font>,
    appearance: Appearance,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    advanced_shape: bool,
) where
    Renderer: text::Renderer,
{
    let bounds = layout.bounds();

    let x = match horizontal_alignment {
        alignment::Horizontal::Left => bounds.x,
        alignment::Horizontal::Center => bounds.center_x(),
        alignment::Horizontal::Right => bounds.x + bounds.width,
    };

    let y = match vertical_alignment {
        alignment::Vertical::Top => bounds.y,
        alignment::Vertical::Center => bounds.center_y(),
        alignment::Vertical::Bottom => bounds.y + bounds.height,
    };

    renderer.fill_text(crate::Text {
        content,
        size: size.unwrap_or_else(|| renderer.default_size()),
        bounds: Rectangle { x, y, ..bounds },
        color: appearance.color.unwrap_or(style.text_color),
        font: font.unwrap_or_else(|| renderer.default_font()),
        horizontal_alignment,
        vertical_alignment,
        advanced_shape,
    });
}

impl<'a, Message, Renderer> From<Text<'a, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: text::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(text: Text<'a, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(text)
    }
}

impl<'a, Renderer> Clone for Text<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            size: self.size,
            width: self.width,
            height: self.height,
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,
            font: self.font,
            style: self.style.clone(),
            advanced_shape: self.advanced_shape,
        }
    }
}

impl<'a, Renderer> From<&'a str> for Text<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(content: &'a str) -> Self {
        Self::new(content)
    }
}

impl<'a, Message, Renderer> From<&'a str> for Element<'a, Message, Renderer>
where
    Renderer: text::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(content: &'a str) -> Self {
        Text::from(content).into()
    }
}

/// The style sheet of some text.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default + Clone;

    /// Produces the [`Appearance`] of some text.
    fn appearance(&self, style: Self::Style) -> Appearance;
}

/// The apperance of some text.
#[derive(Debug, Clone, Copy, Default)]
pub struct Appearance {
    /// The [`Color`] of the text.
    ///
    /// The default, `None`, means using the inherited color.
    pub color: Option<Color>,
}
