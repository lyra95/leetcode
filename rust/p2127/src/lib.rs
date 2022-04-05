use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let mut g = Graph::new(favorite);
        g.go();
        let ans1 = g.max_cycle_size;
        let ans2 = g.number_of_2cycles * 2 + g.sum_of_arms_length;
        max(ans1, ans2) as i32
    }
}

pub struct Graph {
    edges: Vec<usize>,
    edges_reversed: Vec<Vec<usize>>,
    visited: Vec<bool>,
    visited_reversed: Vec<bool>,
    size: usize,
    max_cycle_size: usize,
    number_of_2cycles: usize,
    sum_of_arms_length: usize,
}

impl Graph {
    pub fn new(edges: Vec<i32>) -> Self {
        let n = edges.len();
        let edges: Vec<usize> = edges.iter().map(|x| *x as usize).collect();
        let mut edges_reversed: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (v1, v2) in edges.iter().enumerate() {
            let a = &mut (edges_reversed[*v2]);
            a.push(v1);
        }
        Self {
            edges,
            edges_reversed,
            visited: vec![false; n],
            visited_reversed: vec![false; n],
            size: n,
            max_cycle_size: 0,
            number_of_2cycles: 0,
            sum_of_arms_length: 0,
        }
    }

    pub fn go(&mut self) {
        for node in 0..self.size {
            if !self.visited[node] {
                self.go_straight(&node);
            }
        }
    }

    fn go_straight(&mut self, start: &usize) {
        let mut path: Vec<usize> = Vec::new();
        let mut cur = *start;
        while !self.visited[cur] {
            self.visited[cur] = true;
            path.push(cur);
            cur = self.edges[cur];
        }

        // there must be a cycle
        let cycle_start = path.iter().position(|&node| node == cur).unwrap();
        let cycle = &path[cycle_start..];

        self.max_cycle_size = max(cycle.len(), self.max_cycle_size);
        if cycle.len() == 2 {
            self.number_of_2cycles += 1;
            self.sum_of_arms_length += self.get_arm_len(&cycle[0], &cycle[1]);
        } else {
            self.dfs(&cycle[0]);
        }
    }

    fn dfs(&mut self, start: &usize) -> usize {
        self.visited_reversed[*start] = true;
        self.visited[*start] = true;
        let mut depth = 0;
        let nexts = self.edges_reversed[*start].clone();
        for next in nexts.iter() {
            if !self.visited_reversed[*next] {
                depth = max(self.dfs(next) + 1, depth);
            }
        }
        depth
    }

    fn get_arm_len(&mut self, left: &usize, right: &usize) -> usize {
        self.visited_reversed[*left] = true;
        self.visited[*left] = true;
        self.visited_reversed[*right] = true;
        self.visited[*right] = true;

        self.dfs(left) + self.dfs(right)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::maximum_invitations(vec![2, 2, 1, 2]), 3);
        assert_eq!(Solution::maximum_invitations(vec![3, 0, 1, 4, 1]), 4);
        assert_eq!(
            Solution::maximum_invitations(vec![1, 0, 0, 2, 1, 4, 7, 8, 9, 6, 7, 10, 8]),
            6
        );
        assert_eq!(
            Solution::maximum_invitations(vec![
                7, 0, 7, 13, 11, 6, 8, 5, 9, 8, 9, 14, 15, 7, 11, 6
            ]),
            11
        );
        assert_eq!(
            Solution::maximum_invitations(vec![
                5, 3, 8, 20, 0, 0, 5, 18, 9, 4, 23, 15, 18, 8, 22, 2, 5, 15, 15, 3, 15, 10, 14, 19
            ]),
            16
        );
    }
}
