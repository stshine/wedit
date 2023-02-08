use std::collections::HashMap;

use app_units::Au;
use webrender::Transaction;
use webrender_api::{FontKey, FontInstanceKey, DocumentId, units::DeviceIntSize};

use super::{inline::FaceInfo, Size};


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

    // fn add_font(&mut self, face_info: &FaceInfo) {
    //     let font_data = std::fs::read(&face_info.path).unwrap();

    //     let mut txn = Transaction::new();
    //     let font_key = self.api.generate_font_key();

    //     txn.add_raw_font(font_key, font_data.clone(), face_info.index);
    //     self.api.send_transaction(self.document_id, txn);
    //     self.font_keys.insert(face_info.clone(), font_key);
    //     self.fonts.insert(face_info.clone(), font_data);
    // }

    pub fn get_font(&mut self, face_info: &FaceInfo) -> &[u8] {
        self.fonts.entry(face_info.clone()).or_insert_with(||std::fs::read(&face_info.path).unwrap())
    }

    pub fn get_font_instance_key(&mut self, face_info: &FaceInfo, font_size: u8) -> Option<FontInstanceKey>{
        self.font_instance_keys.get(&(face_info.clone(), font_size)).map(|k| k.to_owned())
    }
}

pub struct LayoutContext {
    font_cache: FontCache,
    // image_cache: ImageCache,
    viewport_size: Size<Au>,
    pub webrender_api: webrender::RenderApi,
    pub document_id: DocumentId,
}

impl LayoutContext {
    pub fn new(webrender_api: webrender::RenderApi, viewport_size: Size<Au>) -> LayoutContext {
        let size = DeviceIntSize::new(viewport_size.width.to_px(), viewport_size.height.to_px());
        let document_id = webrender_api.add_document(size);

        LayoutContext {
            font_cache: FontCache::new(),
            // image_cache: ImageCache::new(),
            viewport_size,
            webrender_api,
            document_id,
        }
    }

    pub fn get_font(&mut self, face_info: &FaceInfo) -> &[u8] {
        // let font_cache = self.font_cache.borrow();
        // font_cache.fonts.get(face_info).map(|x| x.as_slice());

        if  self.font_cache.fonts.get(face_info).is_none() {
            let font_data = std::fs::read(&face_info.path).unwrap();

            let mut txn = Transaction::new();
            let font_key = self.webrender_api.generate_font_key();

            txn.add_raw_font(font_key, font_data.clone(), face_info.index);
            self.webrender_api.send_transaction(self.document_id, txn);
            self.font_cache.fonts.insert(face_info.clone(), font_data);
            self.font_cache.font_keys.insert(face_info.clone(), font_key);
        }
        self.font_cache.fonts.get(face_info).unwrap()
    }

    pub fn get_font_instance(&mut self, face_info: &FaceInfo, font_size: u8) -> FontInstanceKey {
        // let Some(font_key) = self.font_cache.borrow().font_keys.get(face_info) else {
        //     self.get_font(face_info);
        // }
        if self.font_cache.font_keys.get(face_info).is_none() {
            self.get_font(face_info);
        }
        if let Some(instance_key) = self.font_cache.font_instance_keys.get(&(face_info.clone(), font_size)) {
            return *instance_key;
        } else {
            let font_key = self.font_cache.font_keys.get(face_info).unwrap();
            let mut txn = Transaction::new();
            let instance_key = self.webrender_api.generate_font_instance_key();
            txn.add_font_instance(instance_key, *font_key, font_size as f32, None, None, vec![]);
            self.webrender_api.send_transaction(self.document_id, txn);
            self.font_cache.font_instance_keys.insert((face_info.clone(), font_size), instance_key);
            instance_key
        }
    }
}
