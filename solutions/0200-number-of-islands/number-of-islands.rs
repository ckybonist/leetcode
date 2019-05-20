// Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
//
// Example 1:
//
//
// Input:
// 11110
// 11010
// 11000
// 00000
//
// Output: 1
//
//
// Example 2:
//
//
// Input:
// 11000
// 11000
// 00100
// 00011
//
// Output: 3
//


impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        
        if grid.len() == 0 {
            return count;
        }
        
        let mut g = vec![vec!['0'; grid[0].len()]; grid.len()];
        for row in 0..g.iter().len() {
            g[row].copy_from_slice(grid[row].as_slice());
        }
        for row in 0..g.len() {
            for col in 0..g[0].len() {
                if g[row][col] == '1' {
                    count += 1;
                    Self::flip_adj_grids(row, col, &mut g);
                }
            }
        }
        
        return count;
    }
    
    /** Ensure island is only count once by flipping adjacent its grids to 0 */
    pub fn flip_adj_grids(row: usize, col: usize, grid: &mut Vec<Vec<char>>) {
        if grid[row][col] == '0' {
            return;
        }
        
        grid[row][col] = '0';
        
        if row < grid.len()-1 {
            Self::flip_adj_grids(row + 1, col, grid);
        }
        if row > 0 {
            Self::flip_adj_grids(row - 1, col, grid);
        }
        if col < grid[0].len()-1 {
            Self::flip_adj_grids(row, col + 1, grid);
        }
        if col > 0 {
            Self::flip_adj_grids(row, col - 1, grid);
        }
    }
}
