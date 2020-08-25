pub struct Node {
    partition_line: (u16, u16),
    partition_line_delta: (u16, u16),
    right_bounding_box: (u16, u16, u16, u16),
    left_bounding_box: (u16, u16, u16, u16),
    right_child: u16,
    left_child: u16,
}
