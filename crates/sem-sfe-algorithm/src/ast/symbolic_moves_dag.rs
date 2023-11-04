// struct SymbolicMovesDag {

//     arena: ArenaOfFormulas<LogicFormula>,
//     basis_map: HashMap<String, usize>,
//     formulas: Vec<usize>
// }

// impl 
// /// Credits to:
// /// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
// /// 
// #[derive(Debug)]
// struct Node<T>
// where
//     T: PartialEq,
// {
//     idx: usize,
//     val: T,
//     parent: Option<usize>,
//     children: Vec<usize>,
// }

// impl<T> Node<T>
// where
//     T: PartialEq,
// {
//     fn new(val: T) -> Self {
//         Self {
//             val,
//             parent: None,
//             children: vec![],
//         }
//     }
// }

// #[derive(Debug, Default)]
// struct ArenaOfFormulas<T>
// where
//     T: PartialEq,
// {
//     arena: Vec<Node<T>>,
// }

// impl<T> ArenaTree<T>
// where
//     T: PartialEq,
// {
//     fn node(&mut self, val: T) -> usize {
//         //first see if it exists
//         for node in &self.arena {
//             if node.val == val {
//                 return node.idx;
//             }
//         }
//         // Otherwise, add new node
//         let idx = self.arena.len();
//         self.arena.push(Node::new(idx, val));
//         idx
//     }
// }

// impl<LogicFormula> ArenaTree<LogicFormula>
// {
//     fn node(&mut self, val: ) -> usize {
//         //first see if it exists
//         for node in &self.arena {
//             if node.val == val && n {
//                 return node.idx;
//             }
//         }
//         // Otherwise, add new node
//         let idx = self.arena.len();
//         self.arena.push(Node::new(idx, val));
//         idx
//     }
// }

// #[derive[PartialEq, Eq, Debug]]
// enum LogicFormula {
//     And,
//     Or,
//     BasisElem(usize, usize)
// }