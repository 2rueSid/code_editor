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

    pub fn get_lines_after(&self, ln: usize) -> Option<String> {
        if let (Some(front), Some(back)) = (self.front(), self.back()) {
            if front.line_number <= ln && back.line_number > ln {
                let mut res = String::new();

                for n in self.nodes.iter() {
                    if n.line_number >= ln {
                        res.push_str(n.value.as_str());
                    }
                }

                return Some(res);
            }
        }
        None
    }
    pub fn insert_at(&mut self, ln: usize, new_node: &String, x: u16) {
        let mut found = false;
        let mut new_segment: VecDeque<SegmentNode> = VecDeque::with_capacity(self.nodes.capacity());

        for (i, n) in self.nodes.iter().enumerate() {
            if n.line_number == ln {
                if x == 1 {
                    new_segment.push_back(SegmentNode {
                        value: String::from("\n"),
                        line_number: ln,
                        offset: n.offset,
                        updated: true,
                    })
                }
            } else if n.line_number > ln && i + 1 <= new_segment.capacity() {
                let b = new_segment.back().expect("should exist").clone();

                if !found {
                    found = true;
                    new_segment.push_back(SegmentNode {
                        value: new_node.clone(),
                        line_number: n.line_number + 1,
                        offset: b.offset + b.value.len() - 1,
                        updated: true,
                    })
                }

                new_segment.push_back(SegmentNode {
                    value: n.value.clone(),
                    line_number: n.line_number + 1,
                    offset: b.offset + b.value.len() - 1,
                    updated: true,
                })
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
