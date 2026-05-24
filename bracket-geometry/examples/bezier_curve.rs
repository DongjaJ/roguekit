use bracket_geometry::prelude::*;
use crossterm::queue;
use crossterm::style::Print;
use std::io::{stdout, Write};

fn main() {
    let curve = Curve::new(vec![Point::new(1, 8), Point::new(4, 1), Point::new(8, 8)]);

    let mut fake_console: Vec<char> = vec!['.'; 100];
    for point in curve.bezier_points(32) {
        if point.x >= 0 && point.x < 10 && point.y >= 0 && point.y < 10 {
            let idx = ((point.y * 10) + point.x) as usize;
            fake_console[idx] = '*';
        }
    }

    for control_point in curve.control_points() {
        let idx = ((control_point.y * 10) + control_point.x) as usize;
        fake_console[idx] = 'o';
    }

    for y in 0..10 {
        let mut line = String::from("");
        let idx = y * 10;
        for x in 0..10 {
            line.push(fake_console[idx + x]);
        }
        line.push('\n');
        queue!(stdout(), Print(&line)).expect("Command fail");
    }
    stdout().flush().expect("Flush Fail");
}
