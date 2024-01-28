use std::collections::VecDeque;
use std::result::Result;

#[derive(Clone, Debug)]
pub struct SegmentNode {
    pub value: String,
    pub line_number: usize,
    pub offset: usize,
    pub updated: bool,
}

impl SegmentNode {
    pub fn new(v: String, ln: usize, ofst: usize) -> SegmentNode {
        SegmentNode {
            value: v,
            line_number: ln,
            offset: ofst,
            updated: false,
        }
    }
}

#[derive(Debug)]
pub struct Segment {
    pub nodes: VecDeque<SegmentNode>,
}

impl Segment {
    pub fn new(cp: usize) -> Segment {
        Segment {
            nodes: VecDeque::with_capacity(cp),
        }
    }

    pub fn construct_segment(&self) -> String {
        let mut lines = String::new();

        for line in self.nodes.iter() {
            lines.push_str(&line.value);
        }

        lines
    }

    pub fn get_line(&self, ln: usize) -> Result<&SegmentNode, String> {
        // as lines are sorted we can use bs
        match self
            .nodes
            .binary_search_by(|node| node.line_number.cmp(&ln))
        {
            Ok(index) => Ok(&self.nodes[index]),
            Err(_) => Err("Not Found".to_string()),
        }
    }

    pub fn update_at(&mut self, ln: usize, val: &String) {
        let idx = match self.get_line_idx(ln) {
            Ok(i) => i,
            Err(_) => return,
        };

        if let Some(node) = self.nodes.get_mut(idx) {
            node.value = val.clone();
        }
    }

    pub fn insert_at(&mut self, ln: usize, new_node: &String) {
        let idx = self.get_line_idx(ln);
        if idx.is_err() {
            return;
        }
        let idx = idx.unwrap();

        let mut temp = VecDeque::new();

        self.nodes.pop_back();
        let mut i = 0;
        if idx > i {
            while i < idx {
                temp.push_back(self.nodes.pop_front().unwrap());
                i += 1;
            }

            let last = &temp.back().unwrap();
            temp.push_back(SegmentNode {
                value: new_node.clone(),
                line_number: ln,
                offset: last.offset + last.value.len(),
                updated: true,
            });
        } else {
            let first = self.nodes.front().unwrap();
            temp.push_back(SegmentNode {
                value: new_node.clone(),
                line_number: ln,
                offset: first.offset,
                updated: true,
            });
            // temp.push_back()
        }

        while let Some(n) = self.nodes.pop_front() {
            temp.push_back(SegmentNode {
                value: n.value,
                line_number: n.line_number + 1,
                offset: n.offset,
                updated: true,
            });
        }

        self.nodes = temp;
    }

    pub fn add_b(&mut self, n: SegmentNode) {
        self.nodes.push_back(n);
    }

    pub fn add_f(&mut self, n: SegmentNode) {
        self.nodes.push_front(n);
    }

    pub fn new_b(&mut self, v: String, ln: usize, ofst: usize) {
        self.add_b(SegmentNode::new(v, ln, ofst))
    }

    pub fn new_f(&mut self, v: String, ln: usize, ofst: usize) {
        self.add_f(SegmentNode::new(v, ln, ofst))
    }

    pub fn back(&self) -> Option<&SegmentNode> {
        self.nodes.back()
    }

    pub fn front(&self) -> Option<&SegmentNode> {
        self.nodes.front()
    }

    pub fn pop_f(&mut self) {
        self.nodes.pop_front();
    }

    pub fn pop_b(&mut self) {
        self.nodes.pop_back();
    }

    fn get_line_idx(&self, ln: usize) -> Result<usize, String> {
        match self
            .nodes
            .binary_search_by(|node| node.line_number.cmp(&ln))
        {
            Ok(index) => Ok(index),
            Err(_) => Err("Not Found".to_string()),
        }
    }
}
