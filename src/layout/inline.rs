use std::path::PathBuf;

use app_units::Au;
use rustybuzz::{Script, UnicodeBuffer, Feature, Face};

use super::context::LayoutContext;
use super::fragment::TextFragment;
use super::{Point, Size};

#[derive(Clone, Copy)]
pub struct GlyphInfo {
    pub glyph_id: u32,
    pub advance: Au,
    pub offset: Point<Au>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FaceInfo {
    pub path: PathBuf,
    pub index: u32,
}

pub struct TextRun {
    pub text: String,
    pub face_info: FaceInfo,
    pub font_size: f32,
    pub script: Script,
    pub rtl: bool,
}

impl TextRun {
    pub fn shape(&self, context: &mut LayoutContext) -> Vec<GlyphInfo> {
        let mut buffer = UnicodeBuffer::new();
        let features: Vec<Feature> = Vec::new();
        let font = context.get_font(&self.face_info);
        let face = Face::from_slice(font, self.face_info.index as u32).unwrap();

        buffer.push_str(&self.text);
        buffer.set_script(self.script);
        if self.rtl {
            buffer.set_direction(rustybuzz::Direction::RightToLeft);
        }
        let glyph_buffer = rustybuzz::shape(&face, &features, buffer);
        let font_size = Au::from_f32_px(self.font_size);
        let upem = face.units_per_em();

        let mut glyphs = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for (info, pos) in glyph_buffer
            .glyph_infos()
            .iter()
            .zip(glyph_buffer.glyph_positions()) {
                let glyph = GlyphInfo {
                    glyph_id: info.glyph_id,
                    advance: font_size * pos.x_advance / upem,
                    offset: Point {
                        i: font_size * pos.x_offset / upem,
                        b: font_size * pos.y_offset / upem,
                    },
                };
                width += pos.x_advance;
                height = std::cmp::max(height, pos.y_offset + pos.y_advance);
                glyphs.push(glyph);
        }
        glyphs
    }
}

pub struct Line {
    fragments: Vec<TextFragment>,
}

pub struct InlineState {
    pub lines: Vec<Line>,
    pub inline_position: Au,
    pub containing_block: Size<Au>
}
