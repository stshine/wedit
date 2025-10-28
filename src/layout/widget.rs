use std::collections::HashMap;

use app_units::Au;
use ttf_parser::{Weight, Style as FontStyle};
use webrender_api::{FontInstanceKey, FontKey};

use super::context::LayoutContext;
use super::{Rect, Size, Sides};
use super::fragment::{TextFragment, BoxFragment, Fragment};
use super::inline::{FaceInfo, TextRun, InlineState};

pub trait Widget {

}

pub struct TextStyle {
    pub color: u32,
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: Weight,
    pub font_style: FontStyle,
    pub line_height: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: 0,
            font_family: "".to_string(),
            font_size: 16.0,
            font_weight: Weight::Normal,
            font_style: FontStyle::Normal,
            line_height: 22.0
        }
    }
}

pub struct Text {
    pub text: String,
    pub style: TextStyle,
}


pub struct FontCache {
    font_keys: HashMap<FaceInfo, FontKey>,
    font_instance_keys: HashMap<(FaceInfo, u8), FontInstanceKey>,
    fonts: HashMap<FaceInfo, Vec<u8>>,
}

impl FontCache {
    pub fn new() -> FontCache {
        FontCache {
            font_keys: HashMap::new(),
            font_instance_keys: HashMap::new(),
            fonts: HashMap::new()
        }
    }

    pub fn get_font(&mut self, face_info: &FaceInfo) -> &[u8] {
        self.fonts.entry(face_info.clone()).or_insert_with(||std::fs::read(&face_info.path).unwrap())
    }

    pub fn get_font_instance_key(&mut self, face_info: &FaceInfo, font_size: u8) -> Option<FontInstanceKey>{
        self.font_instance_keys.get(&(face_info.clone(), font_size)).map(|k| k.to_owned())
    }
}


impl Text {
    pub fn layout(&self, inline_state: &mut InlineState, context: &mut LayoutContext) -> Vec<TextFragment> {
        let line_height = Au::from_f32_px(self.style.line_height);
        let text_runs = self.itemize();
        let mut fragments = Vec::new();

        let mut cur_b = Au(0);

        for run in text_runs {
            let mut infos = run.shape(context);
            let mut remains = Vec::new();

            let mut len = Au(0);

            loop {
                for i in 0..infos.len() {
                    let info = infos[i];
                    if len + info.advance > inline_state.containing_block.width {
                        remains = infos.split_off(i);
                        break;
                    } else {
                        len += info.advance;
                    }
                }

                let rect = Rect {
                    origin: super::Point { i: Au(0), b: cur_b },
                    size: Size { width: len, height: line_height }
                };
                let fragment = TextFragment {
                    font_size: run.font_size,
                    glyphs: infos,
                    rect: rect,
                    face_info: run.face_info.clone(),
                };
                fragments.push(fragment);

                if remains.len() == 0 {
                    break;
                }
                infos = remains;
                remains = Vec::new();
                len = Au(0);
                cur_b += line_height;
            }
        }

        fragments
    }

    pub fn itemize(&self) -> Vec<TextRun> {
        let mut runs = Vec::new();
        let run = TextRun {
            text: self.text.clone(),
            face_info: FaceInfo { path: "resources/FiraCode-Regular.otf".into(), index: 0 },
            font_size: self.style.font_size,
            script: rustybuzz::script::LATIN,
            rtl: false
        };
        runs.push(run);
        runs
    }
}

pub struct Block {
    children: Vec<Text>
}

impl Block {
    pub fn new(children: Vec<Text>) -> Block {
        Block {
            children
        }
    }

    pub fn layout(&self, context: &mut LayoutContext, containing_block: Size<Au>) -> BoxFragment {
        let mut inline_state = InlineState {
            lines: Vec::new(),
            inline_position: Au(0),
            containing_block,
        };

        let mut fragment = BoxFragment {
            rect: Rect {
                origin: super::Point { i: Au(0), b: Au(0) },
                size: containing_block,
            },
            margin: Sides::<Au>::zero(),
            children: Vec::new(),
        };


        for text in &self.children {
            let fragments = text.layout(&mut inline_state, context);
            for frag in fragments {
                fragment.children.push(Fragment::Text(frag));
            }
        }
        fragment
    }
}

pub fn layout_root(root: &Block, context: &mut LayoutContext, viewport_size: Size<Au>) -> BoxFragment {
    root.layout(context, viewport_size)
}
