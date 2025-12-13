fn main() {
    println!("Hello, world!");
}

struct JunctionBox {
    x: isize,
    y: isize,
    z: isize
}
fn vector_distance(j1: JunctionBox, j2: JunctionBox) -> f32 {
    let x2 = (j1.x - j2.x).pow(2);
    let y2 = (j1.y - j2.y).pow(2);
    let z2 = (j1.z - j2.z).pow(2);

    let sq_distance = (x2 + y2 + z2) as f32;

    sq_distance.sqrt()
}

#[cfg(test)]
mod distance_tests {
    use super::*;

    #[test]
    fn it_computes_distance_for_2d() {
        let j1 = JunctionBox{x: 1, y: 1, z: 0};
        let j2 = JunctionBox{x: 2, y: 2, z: 0};
        assert_eq!(vector_distance(j1, j2), 2f32.sqrt());
    }
}


