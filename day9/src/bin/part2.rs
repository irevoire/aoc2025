use aoc::Coord;
use geo::Relate;

fn main() {
    let coords = aoc::parser::lines::<Coord<isize>>()
        .map(|c| geo::Coord {
            x: c.x as f32,
            y: c.y as f32,
        })
        .collect::<Vec<_>>();
    let polygon = geo::geometry::Polygon::new(geo::LineString(coords.clone()), Vec::new());
    let polygon = geo::PreparedGeometry::from(polygon);

    let max = coords
        .iter()
        .flat_map(|left| coords.iter().map(move |right| (left, right)))
        .filter(|(bl, tr)| {
            polygon
                .relate(&geo::PreparedGeometry::from(geo::Rect::<f32>::new(
                    **bl, **tr,
                )))
                .is_covers()
        })
        .map(|(left, right)| {
            ((left.x as usize).abs_diff(right.x as usize) + 1)
                * ((left.y as usize).abs_diff(right.y as usize) + 1)
        })
        .max()
        .unwrap();

    println!("{max}");
}
