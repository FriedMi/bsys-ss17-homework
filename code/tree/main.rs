#[derive(Debug,Clone)]
enum Tree {
    Leaf(i32),
    Internal { children: Vec<Tree>, data: i32 },
}

impl Tree {
    /// Determines if this tree is just a leaf node.
    fn is_leaf(&self) -> bool {
        match *self {
            Tree::Leaf(_) => true,
            Tree::Internal { .. } => false,
        }
    }

    /// Returns the value attached to this node.
    fn data(&self) -> i32 {
        match *self {
            Tree::Leaf(n) => n,
            Tree::Internal { data, .. } => data,
        }
    }

    /// Returns the value attached to this node to mutate
    fn data_mut(&mut self) -> &mut i32 {
        match *self {
            Tree::Leaf(ref mut n) => n,
            Tree::Internal { ref mut data, .. } => data,
        }
    }

    /// Adds another tree as a child of this node.
    fn add_child(&mut self, child: Tree) {
        if let Tree::Leaf(data) = *self {
            *self = Tree::Internal {
                data: data,
                children: vec![],
            }
        }
        match *self {
            Tree::Internal { ref mut children, .. } => {
                children.push(child);
            }
            _ => unreachable!(),
        }
    }

    /// Adds another leaf node with the given value as child of this node.
    fn add_leaf_child(&mut self, child_data: i32) {
        self.add_child(Tree::Leaf(child_data));
    }

    /// If this node is internal, this function returns a slice of all
    /// children.
    fn children(&self) -> Option<&[Tree]> {
        match *self {
            Tree::Internal { ref children, .. } => Some(children),
            _ => None,
        }
    }

    fn children_mut(&mut self) -> Option<&mut [Tree]> {
        match *self {
            Tree::Internal { ref mut children, .. } => Some(children),
            _ => None,
        }
    }

    /// Returns a string that contains the values of all nodes in preorder. The
    /// values are separated by `separator`.
    fn format_preorder(&self, separator: char) -> String {
        match *self {
            Tree::Leaf(data) => data.to_string(),
            Tree::Internal { data, ref children } => {
                let mut out = data.to_string();
                for child in children {
                    out.push(separator);
                    out = out + &child.format_preorder(separator);
                }
                out
            }
        }
    }
}


fn main() {
    let mut t1 = Tree::Leaf(815);
    let mut t2 = Tree::Internal {
        children: (vec![Tree::Leaf(27), Tree::Leaf(42)]),
        data: 9,
    };

    println!("{}", t1.data());
    println!("{}", t2.data());


    assert_eq!(t1.is_leaf(), true);
    assert_eq!(t2.is_leaf(), false);

    println!("\nNow incrment each +1");
    *t1.data_mut() += 1;
    *t2.data_mut() = t2.data() + 1; //other kind
    println!("{}", t1.data());
    println!("{}", t2.data());


    println!("\nTest Methods in for loop");
    let a = Tree::Leaf(3);
    let b = Tree::Leaf(7);
    let mut root = Tree::Internal {
        children: vec![a, b],
        data: 99,
    };

    //Print all child of root
    for child in root.children().unwrap() {
        println!("{:#?}", child);
    }

    //Add more leafes to childs
    for child in root.children_mut().unwrap() {
        child.add_leaf_child(100);
        child.add_leaf_child(200);
    }

    println!("Preordered output of root");
    println!("{:#?}", root);

    let t4 = Tree::Leaf(815);
    println!("{}", t4.format_preorder('-'));

    println!("{}", root.format_preorder('-'));


    //String Test
    //let s1 = "Hello".to_string();
    //let s2 = " World".to_string();
    //let s = s1 + &s2;
}
