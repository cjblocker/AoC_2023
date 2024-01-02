//! Day 25: Snowverload
use nalgebra::base::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn day25_p1<const N: usize>(data: &str) -> usize {
    let mut adj_matrix = Box::new([[0u8; N]; N]);
    let mut index = 0usize;
    let mut index_map = HashMap::new();
    for line in data.lines() {
        let (left, rights) = line.split_once(": ").unwrap();
        let left_i = *index_map.entry(left).or_insert_with(|| {
            let i = index;
            index += 1;
            i
        });
        for right in rights.split(' ') {
            let right_i = *index_map.entry(right).or_insert_with(|| {
                let i = index;
                index += 1;
                i
            });
            adj_matrix[left_i][right_i] = 1;
            adj_matrix[right_i][left_i] = 1;
        }
    }
    let degree: Vec<u8> = adj_matrix.iter().map(|row| row.iter().sum()).collect();

    // I tried to avoid heavy dependencies, but I don't know how to solve this one outside
    // of an eigen-decomposition of the Laplacian. I'm not using the fact that I should
    // only cut 3 edges. Seems like there is a better way, but this is plenty fast in
    // release mode, even though the matrix is not static.
    let mut lap_matrix = DMatrix::from_fn(N, N, |row, col| -(adj_matrix[row][col] as f32));
    for (ii, val) in degree.into_iter().enumerate() {
        lap_matrix[(ii, ii)] = val as f32;
    }
    let edecom = lap_matrix.symmetric_eigen();
    let mut eigen: Vec<(_, _)> = edecom
        .eigenvalues
        .into_iter()
        .zip(edecom.eigenvectors.column_iter())
        .collect();
    eigen.sort_by(|x, y| x.0.partial_cmp(y.0).unwrap());
    let (_, fielder) = eigen[1];
    let set1 = fielder.into_iter().filter(|&x| *x > 0.0).count();
    (N - set1) * set1
}

pub fn run_day25_p1() -> usize {
    let filename = "data/day_25.txt";
    let data = read_to_string(filename).unwrap();
    // sort of cheating here using a const for
    // something I had to compute from the input
    day25_p1::<1476>(&data)
}

fn main() {
    let now = Instant::now();
    let sol = run_day25_p1();
    let elapsed = now.elapsed().as_millis();
    println!("Day 25 part 1 solution is: {sol} in {elapsed} ms");
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        jqt: rhn xhk nvd\n\
        rsh: frs pzl lsr\n\
        xhk: hfx\n\
        cmg: qnr nvd lhk bvb\n\
        rhn: xhk bvb hfx\n\
        bvb: xhk hfx\n\
        pzl: lsr hfx nvd\n\
        qnr: nvd\n\
        ntq: jqt hfx bvb xhk\n\
        nvd: lhk\n\
        lsr: lhk\n\
        rzs: qnr cmg lsr rsh\n\
        frs: qnr lhk lsr";

    #[test]
    fn test_day25_p1_example() {
        assert_eq!(day25_p1::<15>(EXAMPLE), 54);
    }

    #[test]
    fn test_day25_p1() {
        assert_eq!(run_day25_p1(), 544523);
    }
}
