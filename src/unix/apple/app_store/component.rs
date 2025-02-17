// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ComponentExt, ComponentsExt};

#[doc = include_str!("../../../../md_doc/component.md")]
pub struct Component {}

impl ComponentExt for Component {
    fn temperature(&self) -> f32 {
        0.0
    }

    fn max(&self) -> f32 {
        0.0
    }

    fn critical(&self) -> Option<f32> {
        None
    }

    fn label(&self) -> &str {
        ""
    }

    fn refresh(&mut self) {}
}

#[doc = include_str!("../../../../md_doc/components.md")]
pub struct Components {
    components: Vec<Component>,
}

impl ComponentsExt for Components {
    fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    fn components(&self) -> &[Component] {
        &self.components
    }

    fn components_mut(&mut self) -> &mut [Component] {
        &mut self.components
    }

    fn refresh_list(&mut self) {
        // Doesn't do anything.
    }
}
