#[derive(Clone)]
pub struct SegmentNode {
    pub value: String,
    pub line_number: usize,
    pub offset: usize,
}
