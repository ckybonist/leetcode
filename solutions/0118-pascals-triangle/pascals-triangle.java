// Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.
//
//
// In Pascal's triangle, each number is the sum of the two numbers directly above it.
//
// Example:
//
//
// Input: 5
// Output:
// [
//      [1],
//     [1,1],
//    [1,2,1],
//   [1,3,3,1],
//  [1,4,6,4,1]
// ]
//
//


class Solution {
    public List<List<Integer>> generate(int numRows) {
        List<List<Integer>> triangle = new ArrayList<List<Integer>>(numRows);
        
        if (numRows == 0) {
            return triangle;
        }
        
        for (int i = 0; i < numRows; i++) {
            List<Integer> row = new ArrayList(i+1);
            
            for (int j = 0; j <= i; j++) {
                if (i < 2 || j == 0 || i == j) {
                    row.add(1);
                } else {
                    List<Integer> prevRow = triangle.get(i - 1);
                    row.add(prevRow.get(j - 1) + prevRow.get(j));
                }
            }
            
            triangle.add(row);
        }
        
        return triangle;
    }
}
