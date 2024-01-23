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
        match self
            .nodes
            .binary_search_by(|node| node.line_number.cmp(&ln))
        {
            Ok(index) => Ok(&self.nodes[index]),
            Err(_) => Err("Not Found".to_string()),
        }
    }

    pub fn insert_and_shift(&mut self, ln: usize, new_node: &String) {
        let back_n = self.back().expect("Back should exist").clone();
        if back_n.line_number == ln {
            // if is the last node, just pop first and push a new node to the end
            self.pop_f();
            self.new_b(
                new_node.to_string(),
                ln + 1,
                back_n.offset + back_n.value.len() - 1,
            );
            return;
        }

        let mut found = false;
        let mut new_segment: VecDeque<SegmentNode> = VecDeque::with_capacity(self.nodes.capacity());

        for n in self.nodes.iter() {
            if n.line_number >= ln + 1 && new_segment.len() <= new_segment.capacity() {
                if !found {
                    found = true;
                    new_segment.push_back(SegmentNode {
                        value: new_node.clone(),
                        line_number: n.line_number,
                        offset: n.offset,
                        updated: true,
                    });
                }
                let mut updated_n = n.clone();
                updated_n.updated = true;
                updated_n.line_number = n.line_number + 1;
                updated_n.offset = n.offset + new_node.len() - 1;
                new_segment.push_back(updated_n);
            } else {
                new_segment.push_back(n.clone());
            }
        }

        self.nodes = new_segment;
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
}
