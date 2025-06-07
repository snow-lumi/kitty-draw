

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SelectSingleState {
    Selection(usize),
}

#[expect(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct SelectSingleOptions {
    pub line: SelectLineOptions,
}

#[expect(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct SelectLineOptions {
    pub expand: LineExpandDirection,
    pub expand_x: LineExpandDirection,
    pub expand_y: LineExpandDirection,
}

#[expect(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LineExpandDirection {
    Start,
    End,
    Both,
}