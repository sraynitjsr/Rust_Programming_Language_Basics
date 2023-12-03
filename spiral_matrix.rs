fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let (mut top, mut bottom, mut left, mut right) = (0, matrix.len() - 1, 0, matrix[0].len() - 1);

    while top <= bottom && left <= right {
        // Traverse top row
        for j in left..=right {
            result.push(matrix[top][j]);
        }
        top += 1;

        // Traverse right column
        for i in top..=bottom {
            result.push(matrix[i][right]);
        }
        right -= 1;

        // Traverse bottom row
        if top <= bottom {
            for j in (left..=right).rev() {
                result.push(matrix[bottom][j]);
            }
            bottom -= 1;
        }

        // Traverse left column
        if left <= right {
            for i in (top..=bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
        }
    }

    result
}

fn main() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    let result = spiral_order(matrix);
    println!("{:?}", result);
}
