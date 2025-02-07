use crate::{damage::Category, element::ElementalApplication};

use super::stats::Stat;

#[derive(Clone)]
pub struct Talent {
    category: Category,
    elem_app: Option<ElementalApplication>,
    scalings: Vec<Stat>,
}
impl Talent {
    pub fn new(
        category: Category,
        elem_app: Option<ElementalApplication>,
        scalings: Vec<Stat>,
    ) -> Self {
        Self {
            category,
            elem_app,
            scalings,
        }
    }

    pub fn category(&self) -> Category {
        self.category
    }
    pub fn get_scalings(&self) -> &[Stat] {
        &self.scalings
    }
    pub fn elem_app(&self) -> Option<&ElementalApplication> {
        self.elem_app.as_ref()
    }
}
