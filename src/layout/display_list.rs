use app_units::Au;
use webrender_api::PipelineId;

use super::{fragment::{TextFragment, BoxFragment, Fragment}, Rect, Point, context::LayoutContext};

pub struct DisplayListBuilder<'a> {
    pub scale_factor: f32,
    space_and_clip: webrender_api::SpaceAndClipInfo,
    context: &'a mut LayoutContext,
    pub wr: webrender_api::DisplayListBuilder,
}

impl<'a> DisplayListBuilder<'a> {
    pub fn new(scale_factor: f32, pipeline_id: PipelineId, context: &'a mut LayoutContext) -> Self {
        DisplayListBuilder {
            scale_factor,
            space_and_clip: webrender_api::SpaceAndClipInfo::root_scroll(pipeline_id),
            context,
            wr: webrender_api::DisplayListBuilder::new(pipeline_id),
        }
    }

    fn common_properties(&self, clip_rect: Rect<Au>) -> webrender_api::CommonItemProperties {
        webrender_api::CommonItemProperties {
            clip_rect: clip_rect.to_layout(self.scale_factor),
            clip_chain_id: self.space_and_clip.clip_chain_id,
            spatial_id: self.space_and_clip.spatial_id,
            flags: webrender_api::PrimitiveFlags::default(),
        }
    }
}

impl BoxFragment {
    pub fn build_display_list(&self, builder: &mut DisplayListBuilder, containing_block: Rect<Au>) {
        // FIXME: build for margins.
        let containing_block = Rect {
            origin: containing_block.origin + self.rect.origin,
            size: self.rect.size
        };

        for fragment in &self.children {
            match fragment {
                Fragment::Text(text_fragment) => {
                    text_fragment.build_display_list(builder, containing_block);
                }
                Fragment::Box(box_fragment) => {
                    box_fragment.build_display_list(builder, containing_block);
                }
            }
        }

        // builder.wr.push_rect(
        //     &builder.common_properties(containing_block),
        //     containing_block.to_layout(builder.scale_factor),
        //     webrender_api::ColorF::new(1.0, 0.0, 0.0, 1.0)
        // );
    }
}

impl TextFragment {
    pub fn build_display_list(&self, builder: &mut DisplayListBuilder, containing_block: Rect<Au>) {
        let font_key = builder.context.get_font_instance(
            &self.face_info,
            (self.font_size * builder.scale_factor) as u8
        );
        let color = webrender_api::ColorF::new(0.0, 0.0, 0.0, 1.0);
        // let color = webrender_api::ColorF::new(0.0, 0.0, 0.0, 1.0);
        let common = builder.common_properties(containing_block);
        let bounds = self.rect.translate(containing_block.origin);

        let mut glyphs = Vec::new();

        let mut cur_i = Au(0);
        for glyph in &self.glyphs {
            // The y axis of a glyph starts from downside up.
            let point_au = Point::new(cur_i, bounds.origin.b + bounds.size.height) + glyph.offset;
            glyphs.push(webrender_api::GlyphInstance {
                index: glyph.glyph_id,
                point: point_au.to_layout(builder.scale_factor),
            });
            cur_i += glyph.advance;
        }

        builder.wr.push_text(&common, bounds.to_layout(builder.scale_factor), &glyphs, font_key, color, None)
    }
}
