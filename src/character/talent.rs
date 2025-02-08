use crate::{
    damage::{Attribute, Category},
    element::ElementalApplication,
};

use super::stats::{Condition, Stat};

#[derive(Clone)]
pub struct Talent {
    category: Option<Category>,
    elem_app: Option<ElementalApplication>,
    scalings: Vec<Stat>,
}
impl Talent {
    pub fn new(
        category: Option<Category>,
        elem_app: Option<ElementalApplication>,
        scalings: Vec<Stat>,
    ) -> Self {
        Self {
            category,
            elem_app,
            scalings,
        }
    }

    pub fn category(&self) -> Option<Category> {
        self.category
    }
    pub fn get_scalings(&self) -> &[Stat] {
        &self.scalings
    }
    pub fn elem_app(&self) -> Option<&ElementalApplication> {
        self.elem_app.as_ref()
    }
    pub fn attribute(&self) -> Attribute {
        self.elem_app.as_ref().map(|x| x.element()).into()
    }

    /// Returns all conditions that this talent meets
    pub fn conditions_met(&self) -> Vec<Option<Condition>> {
        let mut result = vec![None, Some(Condition::Attribute(self.attribute()))];
        if let Some(category) = self.category() {
            result.push(Some(Condition::Category(category)));
        }
        result
    }
}
