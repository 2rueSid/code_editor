use std::collections::VecDeque;
use std::result::Result;

#[derive(Clone, Debug)]
pub struct SegmentNode {
    pub value: String,
    pub line_number: usize,
    pub offset: usize,
}

impl SegmentNode {
    pub fn new(v: String, ln: usize, ofst: usize) -> SegmentNode {
        SegmentNode {
            value: v,
            line_number: ln,
            offset: ofst,
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
        match self.nodes.iter().find(|&n| n.line_number == ln) {
            Some(node) => Ok(node),
            None => Err("Not Found".to_string()),
        }
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
