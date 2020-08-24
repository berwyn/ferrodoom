pub(crate) struct Node {
    partition_line: (i16, i16),
    partition_line_delta: (i16, i16),
    right_bounding_box: (i16, i16, i16, i16),
    left_bounding_box: (i16, i16, i16, i16),
    right_child: i16,
    left_child: i16,
}
