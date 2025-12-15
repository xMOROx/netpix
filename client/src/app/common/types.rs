use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use eframe::epaint::{Color32, Hsva};

pub struct TableConfig {
    pub row_height: f32,
    pub header_height: f32,
    pub space_after_filter: f32,
}

impl TableConfig {
    pub fn new(row_height: f32, header_height: f32, space_after_filter: f32) -> Self {
        Self {
            row_height,
            header_height,
            space_after_filter,
        }
    }
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            row_height: 25.0,
            header_height: 30.0,
            space_after_filter: 5.0,
        }
    }
}

#[derive(Default)]
pub struct StreamAliasHelper {
    cache: RefCell<HashMap<u32, String>>,
}

impl StreamAliasHelper {
    pub fn get_alias(&self, ssrc: u32) -> String {
        let mut cache = self.cache.borrow_mut();

        if let Some(alias) = cache.get(&ssrc) {
            return alias.clone();
        }

        let index = cache.len() as u32;
        let alias = self.index_to_letter(index);

        cache.insert(ssrc, alias.clone());
        alias
    }

    fn index_to_letter(&self, mut index: u32) -> String {
        let mut result = Vec::with_capacity(4);
        loop {
            let remainder = index % 26;
            result.push((b'A' + remainder as u8) as char);
            if index < 26 {
                break;
            }
            index = (index / 26) - 1;
        }
        result.into_iter().rev().collect()
    }

    pub fn get_color(&self, ssrc: u32) -> Color32 {
        let mut cache = self.cache.borrow_mut();

        let index = if let Some(_) = cache.get(&ssrc) {
            0
        } else {
            let index = cache.len() as u32;
            let alias = self.index_to_letter(index);
            cache.insert(ssrc, alias);
            0
        };

        let hash = (ssrc as u64).wrapping_mul(11400714819323198485);
        let hue = (hash as f32) / (u64::MAX as f32); // 0.0 - 1.0

        // High saturation and value for visibility against dark backgrounds
        // Maybe different logic for dark mode?
        Color32::from(Hsva::new(hue, 0.7, 0.9, 1.0))
    }

    pub fn print_ssrc(&self, ssrc: u32) -> String {
        format!("{:x} | alias: {}", ssrc, self.get_alias(ssrc))
    }
}