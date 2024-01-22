use crate::traits::ArithmeticOps;

pub fn format_matrix<const ROW_COUNT: usize, const COLUMN_COUNT: usize, Number: ArithmeticOps>(
    matrix: &[[Number; COLUMN_COUNT]; ROW_COUNT],
    unit_suffix: &str,
) -> String {
    const EMPTY_STRING: String = String::new();
    let mut rows = [EMPTY_STRING; ROW_COUNT];
    for column_index in 0..COLUMN_COUNT {
        let column = format_column::<ROW_COUNT, COLUMN_COUNT, Number>(matrix, column_index);
        for row_index in 0..ROW_COUNT {
            if column_index == 0 {
                rows[row_index] += "[";
            }
            rows[row_index] += &column[row_index];
            rows[row_index] += if column_index < COLUMN_COUNT - 1 {
                " "
            } else {
                "]"
            };
            if row_index == 0 && column_index == COLUMN_COUNT - 1 {
                rows[row_index] += unit_suffix;
            }
        }
    }
    rows.join("\n")
}

// It receives a matrix of numbers, and a column index,
// processes the numbers of the specified column,
// and returns an array of the lined up formatted numbers.
fn format_column<const ROW_COUNT: usize, const COLUMN_COUNT: usize, Number: ArithmeticOps>(
    matrix: &[[Number; COLUMN_COUNT]; ROW_COUNT],
    column_index: usize,
) -> [String; ROW_COUNT] {
    const EMPTY_STRING: String = String::new();
    let mut padded_cells = [EMPTY_STRING; ROW_COUNT];
    let mut max_dot_pos = 0;
    let mut max_fractional_len = 0;
    let mut formatted_cells = [EMPTY_STRING; ROW_COUNT];
    let mut dot_positions = [0; ROW_COUNT];
    for row_index in 0..ROW_COUNT {
        formatted_cells[row_index] = format!("{}", matrix[row_index][column_index]);
        dot_positions[row_index] =
            if let Some(pos) = formatted_cells[row_index].bytes().position(|c| c == b'.') {
                pos
            } else {
                formatted_cells[row_index].len()
            };
        max_dot_pos = core::cmp::max(max_dot_pos, dot_positions[row_index]);
        max_fractional_len = core::cmp::max(
            max_fractional_len,
            formatted_cells[row_index].len() - dot_positions[row_index],
        );
    }
    for row_index in 0..ROW_COUNT {
        padded_cells[row_index] = format!(
            "{}{}{}",
            " ".repeat(max_dot_pos - dot_positions[row_index]),
            formatted_cells[row_index],
            " ".repeat(
                max_fractional_len + dot_positions[row_index] - formatted_cells[row_index].len()
            ),
        );
    }
    padded_cells
}
