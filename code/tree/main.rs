#[derive(Debug,Clone)]
enum Tree {
    Leaf(i32),
    Internal{
        children: Vec<Tree>,
        data: i32},
}

impl Tree {
    fn is_leaf(&self) -> bool {
        match *self {
            Tree::Leaf(_) => true,
            Tree::Internal{..} => false,
        }
    }

    fn data(&self) -> i32 {
        match *self {
            Tree::Leaf(n) => n,
            Tree::Internal{data,..} => data,
        }
    }

    fn data_mut(&mut self ) -> &mut i32 {
        match *self {
            Tree::Leaf(ref mut n) => n,
            Tree::Internal{ref mut data,..} => data,
        }
    }
}


fn main() {  
    let mut t1 = Tree::Leaf(815);
    let mut t2 = Tree::Internal {
                    children: (vec![
                    Tree::Leaf(27), 
                    Tree::Leaf(42),]),
                    data: 9};

    println!("{}", t1.data());
    println!("{}", t2.data());


    assert_eq!(t1.is_leaf(), true);
    assert_eq!(t2.is_leaf(), false);

    println!("Now incrment each +1");
    *t1.data_mut() += 1;
    *t2.data_mut() = t2.data() +1; //other kind
    println!("{}", t1.data());
    println!("{}", t2.data());
}
