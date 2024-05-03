use std::io;

struct TreeNode {
    value: i32,
    left: Option<usize>,
    right: Option<usize>,
    height: usize,
}

struct AVLTree {
    nodes: Vec<TreeNode>,
}

impl AVLTree {
    fn new() -> Self {
        AVLTree { nodes: vec![] }
    }

    fn insert(&mut self, value: i32) {
        if self.nodes.is_empty() {
            self.nodes.push(TreeNode {
                value,
                left: None,
                right: None,
                height: 1,
            });
        } else {
            let root_index = self.insert_recursive(0, value);
            self.rebalance(root_index);
        }
    }

    fn insert_recursive(&mut self, index: usize, value: i32) -> usize {
        let new_index = if value < self.nodes[index].value {
            if let Some(left_index) = self.nodes[index].left {
                self.insert_recursive(left_index, value)
            } else {
                let new_index = self.nodes.len();
                self.nodes.push(TreeNode {
                    value,
                    left: None,
                    right: None,
                    height: 1,
                });
                self.nodes[index].left = Some(new_index);
                new_index
            }
        } else {
            if let Some(right_index) = self.nodes[index].right {
                self.insert_recursive(right_index, value)
            } else {
                let new_index = self.nodes.len();
                self.nodes.push(TreeNode {
                    value,
                    left: None,
                    right: None,
                    height: 1,
                });
                self.nodes[index].right = Some(new_index);
                new_index
            }
        };

        self.rebalance(index)
    }

    fn rebalance(&mut self, index: usize) -> usize {
        self.update_height(index);

        let balance = self.balance_factor(index);

        if balance > 1 {
            let left_index = self.nodes[index].left.unwrap();
            if self.balance_factor(left_index) < 0 {
                self.nodes[index].left = Some(self.rotate_left(left_index));
            }
            return self.rotate_right(index);
        } else if balance < -1 {
            let right_index = self.nodes[index].right.unwrap();
            if self.balance_factor(right_index) > 0 {
                self.nodes[index].right = Some(self.rotate_right(right_index));
            }
            return self.rotate_left(index);
        }

        index
    }

    fn update_height(&mut self, index: usize) {
        let left_height = self.height(self.nodes[index].left);
        let right_height = self.height(self.nodes[index].right);
        self.nodes[index].height = 1 + left_height.max(right_height);
    }

    fn height(&self, index: Option<usize>) -> usize {
        index.map_or(0, |idx| self.nodes[idx].height)
    }

    fn balance_factor(&self, index: usize) -> i32 {
        let left_height = self.height(self.nodes[index].left) as i32;
        let right_height = self.height(self.nodes[index].right) as i32;
        left_height - right_height
    }

    fn rotate_left(&mut self, index: usize) -> usize {
        let right_index = self.nodes[index].right.unwrap();
        self.nodes[index].right = self.nodes[right_index].left;
        self.nodes[right_index].left = Some(index);
        self.update_height(index);
        self.update_height(right_index);
        right_index
    }

    fn rotate_right(&mut self, index: usize) -> usize {
        let left_index = self.nodes[index].left.unwrap();
        self.nodes[index].left = self.nodes[left_index].right;
        self.nodes[left_index].right = Some(index);
        self.update_height(index);
        self.update_height(left_index);
        left_index
    }

    fn print_tree(&self) {
        if !self.nodes.is_empty() {
            self.print_subtree(0, 0);
        }
    }

    fn print_subtree(&self, idx: usize, depth: usize) {
        if let Some(right_idx) = self.nodes[idx].right {
            self.print_subtree(right_idx, depth + 4);
        }
        println!("{:1$}{2}", "", depth, self.nodes[idx].value);
        if let Some(left_idx) = self.nodes[idx].left {
            self.print_subtree(left_idx, depth + 4);
        }
    }
}

pub fn main() {
    let mut avl_tree = AVLTree::new();
    loop {
        println!("Digite um número para inserir na árvore (ou 'sair' para terminar):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "sair" {
            break;
        }
        if let Ok(num) = input.parse::<i32>() {
            avl_tree.insert(num);
            avl_tree.print_tree();
        } else {
            println!("Entrada inválida. Por favor, digite um número inteiro.");
        }
    }
}
