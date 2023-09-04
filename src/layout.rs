use std::ffi::c_ulong;
use crate::util::{Bounds, Side};
use crate::util::Direction;

type NodeIndex = usize;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Window {
    pub id: c_ulong,
}

impl Window {
    pub fn new(id: c_ulong) -> Self {
        Self {
            id,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct TreeNode {
    index: NodeIndex,
    parent: Option<NodeIndex>,
    bounds: Bounds,
    ty: TreeNodeTy,
}

impl TreeNode {
    fn new(parent: Option<NodeIndex>, bounds: Bounds, ty: TreeNodeTy) -> Self {
        Self {
            index: 0,
            bounds,
            parent,
            ty,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum TreeNodeTy {
    Leaf {
        window: Window,
    },
    Node {
        left: NodeIndex,
        right: NodeIndex,
        focus: Side,
    }
}

#[derive(Debug)]
pub struct WindowTree {
    nodes: Vec<Option<TreeNode>>,
    root: Option<NodeIndex>,
    bounds: Bounds,
}

impl WindowTree {
    pub fn new(bounds: Bounds) -> Self {
        let nodes = vec![None; 20];
        Self {
            nodes,
            root: None,
            bounds,
        }
    }

    pub fn insert(&mut self, new_window: Window) -> Vec<(c_ulong, Bounds)> {
        let mut changed = Vec::new();
        if self.root != None {
            let focused_index = self.get_focused_index().unwrap();
            let focused_node = self.get_node(focused_index);
            let parent_index = focused_node.parent;
            if let TreeNodeTy::Leaf { window } = focused_node.ty {
                let bounds = focused_node.bounds.clone();
                let bounds_left = bounds.split(Side::Left);
                let bounds_right = bounds.split(Side::Right);
                changed.push((window.id, bounds_left.clone()));
                changed.push((new_window.id, bounds_right.clone()));
                let left = self.add_node(TreeNode::new(Some(focused_index), bounds_left, TreeNodeTy::Leaf { window: window.clone() }));
                let right = self.add_node(TreeNode::new(Some(focused_index), bounds_right, TreeNodeTy::Leaf { window: new_window }));
                self.nodes[focused_index] = Some(TreeNode {
                    index: focused_index,
                    parent: parent_index,
                    bounds,
                    ty: TreeNodeTy::Node {
                        left,
                        right,
                        focus: Side::Right,
                    },
                });
            }
        } else {
            let root_index = self.add_node(TreeNode::new(
                None,
                self.bounds.clone(),
                TreeNodeTy::Leaf {
                    window: new_window,
                },
            ));
            self.root = Some(root_index);
            changed.push((new_window.id, self.bounds.clone()));
        }
        return changed;
    }

    pub fn move_focus(&mut self, direction: &Direction) -> Option<c_ulong> {
        if self.root != None {
            let focused_index = self.get_focused_index().unwrap();
            let focused_node = self.get_node(focused_index);
            let side = if *direction == Direction::Left { Side::Left } else { Side::Right };
            if focused_node.parent != None {
                let mut node = self.get_node(focused_node.parent.unwrap());
                while let TreeNodeTy::Node { focus, .. } = &node.ty {
                    if *focus != side {
                        break;
                    }
                    if let Some(parent) = node.parent {
                        node = self.get_node(parent);
                    } else {
                        return None;
                    }
                }
                let node = self.get_node_mut(node.index);
                if let TreeNodeTy::Node { ref mut focus, .. } = node.ty {
                    *focus = side;
                    let focused_node = self.get_focused_node().unwrap();
                    if let TreeNodeTy::Leaf { window } = focused_node.ty {
                        return Some(window.id);
                    }
                }
            }
        }
        None
    }

    pub fn remove_focused_window(&mut self) -> (Option<c_ulong>, Vec<(c_ulong, Bounds)>) {
        if let Some(root_index) = self.root {
            if let TreeNodeTy::Leaf { .. } = self.get_node(root_index).ty {
                self.nodes[root_index] = None;
                self.root = None;
                return (None, Vec::new());
            }
            let focused_node = self.get_focused_node().unwrap();
            let focused_index = focused_node.index;
            let parent_index = focused_node.parent.unwrap();
            let parent = self.get_node(parent_index);
            if let TreeNodeTy::Node { left, right, focus, .. } = &parent.ty {
                let other_index = if *focus == Side::Left { *right } else { *left };
                let new_parent = if let Some(parent_index) = parent.parent {
                    let parent = self.get_node_mut(parent_index);
                    if let TreeNodeTy::Node { ref mut left, ref mut right, focus, .. } = &mut parent.ty {
                        if *focus == Side::Left {
                            *left = other_index;
                        } else {
                            *right = other_index;
                        }
                    }
                    Some(parent_index)
                } else {
                    self.root = Some(other_index);
                    None
                };
                let other_child = self.get_node_mut(other_index);
                other_child.parent = new_parent;
                self.nodes[focused_index] = None;
                self.nodes[parent_index] = None;
                let changed = self.apply_bounds(other_index);
                let focused_node = self.get_focused_node().unwrap();
                if let TreeNodeTy::Leaf { window } = focused_node.ty {
                    return (Some(window.id), changed);
                } else {
                    panic!("Focused node is not a leaf node.");
                }
            }
        }
        return (None, Vec::new());
    }

    pub fn get_focused_window_id(&self) -> Option<c_ulong> {
        if let Some(node) = self.get_focused_node() {
            if let TreeNodeTy::Leaf { window } = node.ty {
                return Some(window.id);
            }
        }
        None
    }

    fn apply_bounds(&mut self, index: NodeIndex) -> Vec<(c_ulong, Bounds)> {
        let mut changed = Vec::new();
        let mut nodes = Vec::new();
        nodes.push(index);
        let mut i = 0;
        while i < nodes.len() {
            let node = self.get_node_mut(nodes[i]);
            let parent_index = node.parent;
            let index = node.index;
            let bounds = if let Some(parent_index) = parent_index {
                let parent = self.get_node(parent_index);
                if let TreeNodeTy::Node { left, .. } = parent.ty {
                    parent.bounds.split(if index == left { Side::Left } else { Side::Right })
                } else {
                    Bounds::new(0, 0, 0, 0)
                }
            } else {
                self.bounds.clone()
            };
            let node = self.get_node_mut(nodes[i]);
            node.bounds = bounds.clone();
            match &node.ty {
                TreeNodeTy::Node { left, right, .. } => {
                    nodes.push(*left);
                    nodes.push(*right);
                }
                TreeNodeTy::Leaf { window } => {
                    changed.push((window.id, bounds.clone()));
                }
            }
            i += 1;
        }
        return changed;
    }

    fn get_focused_index(&self) -> Option<NodeIndex> {
        if let Some(root_index) = self.root {
            let mut node = self.get_node(root_index);
            let mut index = root_index;
            while let TreeNodeTy::Node { left, right, focus, .. } = &node.ty {
                node = self.get_node(if *focus == Side::Left { *left } else { *right });
                index = if *focus == Side::Left { *left } else { *right };
            }
            return Some(index);
        }
        None
    }

    fn get_focused_node(&self) -> Option<&TreeNode> {
        if let Some(root_index) = self.root {
            let mut node = self.get_node(root_index);
            while let TreeNodeTy::Node { left, right, focus, .. } = &node.ty {
                node = self.get_node(if *focus == Side::Left { *left } else { *right });
            }
            return Some(node);
        } else {
            None
        }
    }

    fn get_node(&self, index: NodeIndex) -> &TreeNode {
        let node = self.nodes[index].as_ref().unwrap();
        return node;
    }

    fn get_node_mut(&mut self, index: NodeIndex) -> &mut TreeNode {
        let node = self.nodes[index].as_mut().unwrap();
        return node;
    }

    fn add_node(&mut self, mut node: TreeNode) -> NodeIndex {
        let index = self.get_empty_index();
        node.index = index;
        self.nodes[index] = Some(node);
        return index;
    }

    fn get_empty_index(&mut self) -> NodeIndex {
        for i in 0..self.nodes.len() {
            if self.nodes[i] == None {
                return i as NodeIndex;
            }
        }
        self.nodes.push(None);
        return (self.nodes.len() -  1) as NodeIndex;
    }
}
