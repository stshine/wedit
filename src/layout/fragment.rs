use app_units::Au;

use super::{Rect, Sides, inline::{FaceInfo, GlyphInfo}};

pub struct TextFragment {
    pub rect: Rect<Au>,
    pub face_info: FaceInfo,
    pub font_size: f32,
    pub glyphs: Vec<GlyphInfo>,
    // pub glyphs: Vec<GlyphInstance>
}

pub enum ReplacedContent {
    Canvas,
    Image,
    Svg,
    Video,
    // Iframe
}

pub struct ReplacedFragment {
    pub rect: Rect<Au>,
    pub content: ReplacedContent,
    pub intrisic_width: Au,
    pub intrisic_height: Au,
}

pub struct BoxFragment {
    pub rect: Rect<Au>,
    pub margin: Sides<Au>,
    pub children: Vec<Fragment>,
}

pub enum Fragment {
    Text(TextFragment),
    // Replaced(ReplacedFragment),
    Box(BoxFragment),
}
