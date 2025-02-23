use crate::{backends::traits::{BackendTrait, DrawerTrair}, style::Style};

pub struct Rll<T:BackendTrait> {
    title: String,
    pub backend: T,
    pub main_container: Option<Container>,
    style_links: Vec<StyleLink>,
}

impl<T: BackendTrait> Rll<T> {
    pub fn new(backend: T) -> Self {
        Self {
            title: String::new(),
            backend,
            main_container: None,
            style_links: vec![],
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
    
    pub fn set_main_container(&mut self, container: Container) {
        self.main_container = Some(container);
    }

    pub fn attach_style_to_id(&mut self, id: &str, style: Vec<Style>) {
        self.style_links.push(
            StyleLink {
                id: Some(id.to_string()),
                class: None,
                style,
            }
        )
    }

    pub fn attach_style_to_class(&mut self, class: &str, style: Vec<Style>) {
        self.style_links.push(
            StyleLink {
                id: None,
                class: Some(class.to_string()),
                style,
            }
        );
    }

    pub fn attach_container_by_id(&mut self, parent_container_id: &str, child_container: Container) {
        if let Some(container) = &mut self.main_container {
            Self::find_and_attach(container, parent_container_id, child_container);
        }
    }

    pub fn change_container_contaiment(&mut self, target_id: &str, new_contaiment: &str) {
        if let Some(container) = &mut self.main_container {
            Self::find_and_change_contaiment(container, target_id, new_contaiment);
        }
    }

    pub fn remove_container_by_id(&mut self, container_id: &str) {
        if let Some(container) = &mut self.main_container {
            Self::find_and_remove(container, container_id)
        }
    }

    fn find_and_attach(container: &mut Container, target_id: &str, child_container: Container) {
        if container.id == target_id {
            container.add_child(child_container);
        } else {
            for child in &mut container.childs {
                Self::find_and_attach(child, target_id, child_container.clone());
            }
        }
    }

    fn find_and_change_contaiment(container: &mut Container, target_id: &str, new_contaiment: &str) {
        if container.id == target_id {
            container.contaiment = new_contaiment.to_string();
        } else {
            for child in &mut container.childs {
                Self::find_and_change_contaiment(child, target_id, new_contaiment);
            }
        }
    }

    fn find_and_remove(container: &mut Container, target_id: &str) {
        let mut found: bool = false;
        for child in &mut container.childs {
            if child.id == target_id {
                found = true;
                break;
            }
        }
        if found {
            container.remove_child(target_id);
        } else {
            for child in &mut container.childs {
                Self::find_and_remove(child, target_id);
            }
        }
    }

    // backend related methods
    pub fn init_backend(&mut self) {
        self.backend.init()
    }
    pub fn render(&mut self) {
        if let Some(container) = &self.main_container {
            self.backend.render(container.clone(), self.style_links.clone());
        }
    }
    pub fn display(& self) {
        self.backend.display();
    }
    pub fn close(&self) {
        self.backend.close();
    }
}

#[derive(Clone)]
pub struct StyleLink {
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Vec<Style>,
}

#[derive(Clone)]
pub struct Container {
    pub id: String,
    pub contaiment: String,
    pub classes: Vec<String>,
    childs: Vec<Container>,
}

impl Container {
    pub fn new(
        id: &str,
        contaiment: &str,
        classes: Vec<&str>,
        childs: Vec<Container>,
    ) -> Self {
        Self {
            id: id.to_string(),
            contaiment: contaiment.to_string(),
            classes: classes.iter().map(|el|{el.to_string()}).collect(),
            childs,
        }
    }

    pub fn childs(& self) -> Vec<Container> {
        self.childs.clone()
    }

    // child container methods
    pub fn add_child(&mut self, child_container: Container) {self.childs.push(child_container);}
    pub fn pop_child(&mut self) {if self.childs.len() > 0 {self.childs.pop();}}
    pub fn clear_childs(&mut self) {self.childs = vec![]}
    pub fn remove_child(&mut self, id: &str) {
        for (i, child) in self.childs.clone().iter().enumerate() {
            if child.id.as_str() == id {
                self.childs.remove(i);
            }
        }
    }

    // class methods
    pub fn add_class(&mut self, class: &str) {self.classes.push(class.to_string());}
    pub fn pop_class(&mut self) {if self.classes.len() > 0 {self.classes.pop();}}
    pub fn clear_classes(&mut self) {self.classes = vec![]}

}