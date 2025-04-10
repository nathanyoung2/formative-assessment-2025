use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

/// Represents a bird or group in a tree.
pub enum Node {
    /// A taxinomical grouping for birds.
    Group {
        name: String,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    },
    /// A bird. This must be at the bottom of the tree, therefore has no children.
    Bird {
        name: String,
        scientific_name: String,
        parent: RefCell<Weak<Node>>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Group { name, .. } => write!(f, "{}", name),
            Node::Bird {
                name,
                scientific_name,
                ..
            } => write!(
                f,
                "{name}\n{scientific_name}\n{full_scientific_name}",
                full_scientific_name = self.full_scientific_name().unwrap_or("".to_string()),
            ),
        }
    }
}

#[derive(Debug)]
pub struct NodeTypeError;

impl Node {
    /// Create a new group Node.
    pub fn new_group(name: &str) -> Self {
        Node::Group {
            name: name.to_string(),
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        }
    }

    /// Create a new bird Node.
    pub fn new_bird(name: &str, scientific_name: &str) -> Self {
        Node::Bird {
            name: name.to_string(),
            scientific_name: scientific_name.to_string(),
            parent: RefCell::new(Weak::new()),
        }
    }

    /// Get the parent of a Node.
    pub fn parent(&self) -> &RefCell<Weak<Node>> {
        match self {
            Node::Bird { parent, .. } => parent,
            Node::Group { parent, .. } => parent,
        }
    }

    /// Get the children of a Node.
    pub fn children(&self) -> Result<&RefCell<Vec<Rc<Node>>>, NodeTypeError> {
        match self {
            Node::Group { children, .. } => Ok(children),
            _ => Err(NodeTypeError),
        }
    }

    /// Get the scientific name of a Node.
    /// Note that this will just return the default name if the varient is Node::Group
    pub fn scientific_name(&self) -> &str {
        match self {
            Node::Group { name, .. } => name,
            Node::Bird {
                scientific_name, ..
            } => scientific_name,
        }
    }

    /// Get the common name of a Node.
    pub fn name(&self) -> &str {
        match self {
            Node::Group { name, .. } => name,
            Node::Bird { name, .. } => name,
        }
    }

    /// Get the full scientific name of a bird.
    /// This is essentially a path of all the parents' names
    pub fn full_scientific_name(&self) -> Option<String> {
        Some(format!(
            "{} {}",
            self.parent()
                .borrow()
                .upgrade()?
                .full_scientific_name()
                .unwrap_or("".to_string())
                .trim(),
            self.scientific_name()
        ))
    }

    /// Add a node to a group node.
    /// Returns Err(NodeTypeError) if this function is called on a `Node::Bird` as a `Node::Bird`
    /// has no children.
    pub fn add(self: Rc<Self>, child: Rc<Self>) -> Result<(), NodeTypeError> {
        match &*self {
            Node::Group { children, .. } => {
                *child.parent().borrow_mut() = Rc::downgrade(&self);
                children.borrow_mut().push(child);
            }
            _ => return Err(NodeTypeError),
        }

        Ok(())
    }
}

pub enum BirdsInGroupErr {
    NoGroupExists,
}

/// Holds references to important nodes on the tree.
pub struct BirdTree {
    /// Tree root node
    root: Rc<Node>,

    /// Nodes with only 1 child depth
    direct_parents: Vec<Rc<Node>>,
}

impl BirdTree {
    /// Build a new bird tree.
    /// Returns None if root or one of the direct parents is a `Node::Bird`
    pub fn new(root: Rc<Node>, direct_parents: Vec<Rc<Node>>) -> Option<Self> {
        // assure that root is a group
        if let Node::Bird { .. } = &*root {
            return None;
        }

        // assure that all direct parents are groups
        for node in direct_parents.iter() {
            if let Node::Bird { .. } = &**node {
                return None;
            }
        }

        // build the BirdTree
        Some(Self {
            root,
            direct_parents,
        })
    }

    /// Find a bird node from its scientific name
    pub fn search_by_scientific_name(&self, name: &str) -> Option<Rc<Node>> {
        for group in self.direct_parents.iter() {
            // can safely unwrap due BirdTree assuring that direct_parents only contains groups
            let children = group.children().unwrap().borrow();

            // search through all direct parents to find the bird
            for child in children.iter() {
                if child.scientific_name().to_lowercase() == name.to_lowercase() {
                    return Some(Rc::clone(child));
                }
            }
        }

        None
    }

    /// Find a bird node from its common name
    pub fn search_by_name(&self, name: &str) -> Option<Rc<Node>> {
        for group in self.direct_parents.iter() {
            // can safely unwrap due BirdTree assuring that direct_parents only contains groups
            let children = group.children().unwrap().borrow();

            // search through all direct parents to find the bird
            for child in children.iter() {
                if child.name().to_lowercase() == name.to_lowercase() {
                    return Some(Rc::clone(child));
                }
            }
        }

        None
    }

    fn get_group_with_name(group: Rc<Node>, group_name: &str) -> Option<Rc<Node>> {
        if let Node::Bird { .. } = &*group {
            return None;
        }

        for child in group.children().unwrap().borrow().iter() {
            if let Node::Group { name, .. } = &**child {
                // return the group if the name matches
                if name.to_lowercase() == group_name.to_lowercase() {
                    return Some(Rc::clone(child));
                }

                // otherwise call this function recursively
                return Self::get_group_with_name(Rc::clone(child), group_name);
            }
        }

        None
    }

    /// Recursively get birds in a group.
    /// Birds will get added to the accumulator Vec.
    fn birds_in_group(acc: &mut Vec<Rc<Node>>, group: Rc<Node>) {
        for child in group.children().unwrap().borrow().iter() {
            match &**child {
                Node::Bird { .. } => acc.push(Rc::clone(child)),
                Node::Group { .. } => Self::birds_in_group(acc, Rc::clone(child)),
            }
        }
    }

    /// Get all birds in a group from a group name.
    pub fn birds_in_group_from_name(
        &self,
        group_name: &str,
    ) -> Result<Vec<Rc<Node>>, BirdsInGroupErr> {
        let group = match Self::get_group_with_name(Rc::clone(&self.root), group_name) {
            Some(group) => group,
            None => return Err(BirdsInGroupErr::NoGroupExists),
        };

        let mut birds = vec![];

        Self::birds_in_group(&mut birds, Rc::clone(&group));

        return Ok(birds);
    }
}

pub fn build_tree() -> BirdTree {
    // create bird groups
    let animalia = Rc::new(Node::new_group("Animalia"));
    let chordata = Rc::new(Node::new_group("Chordata"));
    let aves = Rc::new(Node::new_group("Aves"));
    let psittiaciformes = Rc::new(Node::new_group("Psittiaciformes"));
    let apterygiformes = Rc::new(Node::new_group("Apterygiformes"));
    let passeriformes = Rc::new(Node::new_group("Passeriformes"));
    let strigopidae = Rc::new(Node::new_group("Strigopidae"));
    let apterygidae = Rc::new(Node::new_group("Apterygidae"));
    let rhipiduridae = Rc::new(Node::new_group("Rhipiduridae"));
    let meliphagidae = Rc::new(Node::new_group("Meliphagidae"));
    let nestor = Rc::new(Node::new_group("Nestor"));
    let apteryx = Rc::new(Node::new_group("Apteryx"));
    let rhipidura = Rc::new(Node::new_group("Rhipidura"));
    let prosthemadera = Rc::new(Node::new_group("Prosthemadera"));

    // organise tree of groups
    Rc::clone(&animalia).add(Rc::clone(&chordata)).unwrap();
    Rc::clone(&chordata).add(Rc::clone(&aves)).unwrap();
    Rc::clone(&aves).add(Rc::clone(&psittiaciformes)).unwrap();
    Rc::clone(&aves).add(Rc::clone(&apterygiformes)).unwrap();
    Rc::clone(&aves).add(Rc::clone(&passeriformes)).unwrap();
    Rc::clone(&psittiaciformes)
        .add(Rc::clone(&strigopidae))
        .unwrap();
    Rc::clone(&apterygiformes)
        .add(Rc::clone(&apterygidae))
        .unwrap();
    Rc::clone(&passeriformes)
        .add(Rc::clone(&rhipiduridae))
        .unwrap();
    Rc::clone(&passeriformes)
        .add(Rc::clone(&meliphagidae))
        .unwrap();
    Rc::clone(&strigopidae).add(Rc::clone(&nestor)).unwrap();
    Rc::clone(&apterygidae).add(Rc::clone(&apteryx)).unwrap();
    Rc::clone(&rhipiduridae).add(Rc::clone(&rhipidura)).unwrap();
    Rc::clone(&meliphagidae)
        .add(Rc::clone(&prosthemadera))
        .unwrap();

    // add birds to groups
    Rc::clone(&nestor)
        .add(Rc::new(Node::new_bird("Kaka", "meridionalis")))
        .unwrap();
    Rc::clone(&nestor)
        .add(Rc::new(Node::new_bird("Kea", "notabilis")))
        .unwrap();
    Rc::clone(&apteryx)
        .add(Rc::new(Node::new_bird("Little Spotted Kiwi", "owenii")))
        .unwrap();
    Rc::clone(&rhipidura)
        .add(Rc::new(Node::new_bird("Piwakawaka", "fuliginosa")))
        .unwrap();
    Rc::clone(&prosthemadera)
        .add(Rc::new(Node::new_bird("Tui", "novaeseelandiea")))
        .unwrap();

    let tree = BirdTree::new(
        animalia,
        vec![
            nestor.clone(),
            apteryx.clone(),
            rhipidura.clone(),
            prosthemadera.clone(),
        ],
    )
    .expect("Didn't put in invalid values");

    tree
}
