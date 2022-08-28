use graphics::types::Line;

extern crate graphics;
pub struct DebugLine {
    pub line: Line,
    pub angle: f64,
}

pub struct DebugManager {
    debug_lines: Vec<DebugLine>,
    enabled: bool,
}

impl DebugManager {
    pub fn new(enabled: bool) -> DebugManager {
        DebugManager {
            debug_lines: Vec::new(),
            enabled: enabled,
        }
    }

    pub fn add_line(&mut self, line: DebugLine) {
        self.debug_lines.push(line);
    }

    pub fn clear_lines(&mut self) {
        self.debug_lines.clear();
    }

    pub fn get_lines(&self) -> &Vec<DebugLine> {
        &self.debug_lines
    }
}
