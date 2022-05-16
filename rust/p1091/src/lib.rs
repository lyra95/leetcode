use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }

        let g: Graph = Graph::new(grid);
        let start: Vertex = Vertex { i: 0, j: 0 };
        let goal: Vertex = Vertex {
            i: g.n - 1,
            j: g.n - 1,
        };

        let mut shortests: HashMap<Vertex, i32> = HashMap::new();
        shortests.insert(start, 1);

        let mut visited: HashSet<Vertex> = HashSet::new();
        let mut queue: VecDeque<Vertex> = VecDeque::new();
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if visited.contains(&node) {
                continue;
            }

            visited.insert(node);
            for neighbor in g.neighbors(&node) {
                if visited.contains(&neighbor) {
                    continue;
                }

                let w = shortests.get(&node).unwrap().clone();
                let x = shortests.entry(neighbor).or_insert(w + 1);
                if *x > w + 1 {
                    *x = w + 1;
                }
                queue.push_back(neighbor);
            }
        }

        return if let Some(v) = shortests.get(&goal) {
            *v
        } else {
            -1
        };
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Vertex {
    i: usize,
    j: usize,
}

struct Graph {
    n: usize, // grid is n x n
    grid: Vec<Vec<i32>>,
}

impl Graph {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self {
            n: matrix.len(),
            grid: matrix,
        }
    }

    pub fn neighbors(&self, v: &Vertex) -> Vec<Vertex> {
        if v.i >= self.n || v.j >= self.n {
            panic!("out of bound, (i,j,n)=({},{},{})", v.i, v.j, self.n)
        }

        let mut result: Vec<Vertex> = Vec::new();
        for p in -1..2 {
            for q in -1..2 {
                if p == 0 && q == 0 {
                    continue;
                }

                let _i = v.i as isize + p;
                let _j = v.j as isize + q;
                let n = self.n as isize;

                if _i < 0 || _i >= n {
                    continue;
                }

                if _j < 0 || _j >= n {
                    continue;
                }

                let _i = _i as usize;
                let _j = _j as usize;

                if self.grid[_i][_j] != 0 {
                    continue;
                }

                result.push(Vertex { i: _i, j: _j });
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            4
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 1, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 1, 1],
                vec![0, 1, 0, 0, 0, 1],
                vec![1, 0, 0, 1, 0, 1],
                vec![0, 0, 1, 0, 1, 0]
            ]),
            7
        );
    }
}
