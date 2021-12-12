use crate::cell;

#[derive(Debug)]
pub struct BoxCoords {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

pub fn draw_cell(
    dt: &mut raqote::DrawTarget,
    cell_size: i32,
    cll: cell::Cell,
) -> &mut raqote::DrawTarget {
    let mut pb = raqote::PathBuilder::new();
    // if we can make a Cell and BoxCoords out of a node, then we can reuse rendering stuff
    let coords = BoxCoords {
        x1: (cll.column * cell_size + cell_size) as f32,
        x2: ((cll.column + 1) * cell_size + cell_size) as f32,
        y1: (cll.row * cell_size + cell_size) as f32,
        y2: ((cll.row + 1) * cell_size + cell_size) as f32,
    };

    if cll.west.is_none() {
        pb.move_to(coords.x1, coords.y1);
        pb.line_to(coords.x1, coords.y2);
    }
    if cll.south.is_none() {
        pb.move_to(coords.x1, coords.y2);
        pb.line_to(coords.x2, coords.y2);
    }
    if cll.east.is_none() {
        pb.move_to(coords.x2, coords.y2);
        pb.line_to(coords.x2, coords.y1);
    }
    if cll.north.is_none() {
        pb.move_to(coords.x2, coords.y1);
        pb.line_to(coords.x1, coords.y1);
    }
    if !cll.direction_has_link(cell::Direction::North) {
        pb.move_to(coords.x1, coords.y1);
        pb.line_to(coords.x2, coords.y1);
    }
    if !cll.direction_has_link(cell::Direction::East) {
        pb.move_to(coords.x2, coords.y2);
        pb.line_to(coords.x2, coords.y1);
    }

    let path = pb.finish();

    dt.stroke(
        &path,
        &raqote::Source::Solid(raqote::SolidSource {
            r: 0x0,
            g: 0x0,
            b: 0x0,
            a: 0x99,
        }),
        &raqote::StrokeStyle {
            cap: raqote::LineCap::Round,
            join: raqote::LineJoin::Round,
            width: 2.,
            miter_limit: 1.,
            dash_array: vec![],
            dash_offset: 0.,
        },
        &raqote::DrawOptions::new(),
    );

    dt
}
