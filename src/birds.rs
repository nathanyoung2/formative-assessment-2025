use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub trait Node {
    fn get_name(&self) -> &str;
    fn get_scientific_name(&self) -> &str;
    fn get_parent(&self) -> Option<&Weak<RefCell<Group>>>;

    fn display(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.get_scientific_name(),
            self.get_name(),
            self.get_full_scientific_name()
        )
    }

    fn get_full_scientific_name(&self) -> String {
        let path = if let Some(parent) = self.get_parent() {
            if let Some(parent) = parent.upgrade() {
                let parent_ref = parent.borrow();
                let s = parent_ref.get_full_scientific_name();
                s.to_string()
            } else {
                String::from("")
            }
        } else {
            String::from("")
        };

        return (path + " " + self.get_scientific_name()).trim().to_string();
    }
}

pub trait NodeBuilder {
    type Target;
    fn build(self, parent: Weak<RefCell<Group>>) -> Rc<RefCell<Self::Target>>;
}

pub struct Group {
    name: String,
    children: Vec<Rc<RefCell<dyn Node>>>,
    parent: Option<Weak<RefCell<Group>>>,
}

impl Group {
    pub fn new(name: &str, parent: Weak<RefCell<Group>>) -> Self {
        Self {
            name: name.to_string(),
            parent: Some(parent),
            children: Vec::new(),
        }
    }
    /// Create a root group of a tree
    pub fn root(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::new(),
            parent: None,
        }
    }

    /// Add a node to the group.
    /// This function takes in a node builder with a target that must be a node
    /// The 'static bound is used to ensure that the node does not have any lifetime parameters
    /// that live shorter than the length of the program, in the case of this program, none of the
    /// nodes have lifetime parameters at all so this function is executable.
    pub fn add<B, T>(this: Rc<RefCell<Self>>, node_builder: B) -> Rc<RefCell<T>>
    where
        B: NodeBuilder<Target = T>,
        T: Node + 'static,
    {
        // Weak smart pointer is used as it means the value can still be dropped where there is a
        // a reference held between children and parents, if Rc were used, the reference counter
        // would never reach 0.
        let self_weak = Rc::downgrade(&this);

        // build the node with parent
        let node = node_builder.build(self_weak);

        // add the node to the group
        this.borrow_mut().children.push(node.clone());
        node
    }
}

struct GroupBuilder {
    name: String,
}

impl GroupBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl NodeBuilder for GroupBuilder {
    type Target = Group;
    fn build(self, parent: Weak<RefCell<Group>>) -> Rc<RefCell<Group>> {
        Rc::new(RefCell::new(Group::new(&self.name, parent)))
    }
}

impl Node for Group {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_parent(&self) -> Option<&Weak<RefCell<Group>>> {
        self.parent.as_ref()
    }

    fn get_scientific_name(&self) -> &str {
        self.get_name()
    }
}

pub struct Bird {
    name: String,
    scientific_name: String,
    parent: Weak<RefCell<Group>>,
}

impl Bird {
    pub fn new(name: &str, scientific_name: &str, parent: Weak<RefCell<Group>>) -> Self {
        Self {
            name: name.to_string(),
            scientific_name: scientific_name.to_string(),
            parent,
        }
    }
}

impl Node for Bird {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_scientific_name(&self) -> &str {
        &self.scientific_name
    }

    fn get_parent(&self) -> Option<&Weak<RefCell<Group>>> {
        Some(&self.parent)
    }
}

pub struct BirdBuilder {
    name: String,
    scientific_name: String,
}

impl BirdBuilder {
    pub fn new(name: &str, scientific_name: &str) -> Self {
        Self {
            name: name.to_string(),
            scientific_name: scientific_name.to_string(),
        }
    }
}

impl NodeBuilder for BirdBuilder {
    type Target = Bird;
    fn build(self, parent: Weak<RefCell<Group>>) -> Rc<RefCell<Bird>> {
        Rc::new(RefCell::new(Bird::new(
            &self.name,
            &self.scientific_name,
            parent,
        )))
    }
}

impl Deref for Bird {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

/// Holds references to important nodes in the tree for faster searching
pub struct BirdTree {
    /// The root node
    root: Rc<RefCell<Group>>,

    /// Typically leaves would have no children, but here I am defining a leaf as having no groups
    /// as children, only birds
    leaves: Vec<Rc<RefCell<Group>>>,
}

impl BirdTree {
    pub fn new(root: Rc<RefCell<Group>>) -> Self {
        Self {
            root,
            leaves: Vec::new(),
        }
    }

    pub fn add_leaves(&mut self, leaves: Vec<Rc<RefCell<Group>>>) {
        self.leaves.extend(leaves);
    }

    pub fn search_by_name(&self, name: &str) -> Option<Rc<RefCell<dyn Node>>> {
        for group in self.leaves.iter() {
            for bird in group.borrow().children.iter() {
                let bird = bird.clone();
                if bird.borrow().get_name() == name {
                    return Some(bird);
                }
            }
        }
        None
    }
}

pub fn build_tree() -> BirdTree {
    // the clone method is NOT the clone method from the Clone
    // trait, it is Rc::clone(), which means that the underlying value is not actually being
    // cloned, only the reference counter is increasing.
    let animalia = Rc::new(RefCell::new(Group::root("Animalia")));
    let chordata = Group::add(animalia.clone(), GroupBuilder::new("Chordata"));
    let aves = Group::add(chordata.clone(), GroupBuilder::new("Aves"));
    let psittiaciformes = Group::add(aves.clone(), GroupBuilder::new("Psittiaciformes"));
    let apterygiformes = Group::add(aves.clone(), GroupBuilder::new("Apterygiformes"));
    let passeriformes = Group::add(aves.clone(), GroupBuilder::new("Passeriformes"));
    let strigopidae = Group::add(psittiaciformes.clone(), GroupBuilder::new("Strigopidae"));
    let apterygidae = Group::add(apterygiformes.clone(), GroupBuilder::new("Apterygidae"));
    let rhipiduridae = Group::add(passeriformes.clone(), GroupBuilder::new("Rhipiduridae"));
    let meliphagidae = Group::add(passeriformes.clone(), GroupBuilder::new("Meliphagidae"));
    let nestor = Group::add(strigopidae.clone(), GroupBuilder::new("Nestor"));
    let apteryx = Group::add(apterygidae.clone(), GroupBuilder::new("Apteryx"));
    let rhipidura = Group::add(rhipiduridae.clone(), GroupBuilder::new("Rhipidura"));
    let prosthemadera = Group::add(meliphagidae.clone(), GroupBuilder::new("Prosthemadera"));
    let _ = Group::add(nestor.clone(), BirdBuilder::new("Kaka", "meridionalis"));
    let _ = Group::add(nestor.clone(), BirdBuilder::new("Kea", "notabilis"));
    let _ = Group::add(
        apteryx.clone(),
        BirdBuilder::new("Little Spotted Kiwi", "owenii"),
    );
    let _ = Group::add(
        rhipidura.clone(),
        BirdBuilder::new("Piwakawaka", "fuliginosa"),
    );
    let _ = Group::add(
        prosthemadera.clone(),
        BirdBuilder::new("Tui", "novaeseelandiea"),
    );

    let mut tree = BirdTree::new(animalia);
    tree.add_leaves(vec![
        nestor.clone(),
        apteryx.clone(),
        rhipidura.clone(),
        prosthemadera.clone(),
    ]);

    tree
}
