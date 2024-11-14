fn draw_tree(triangle_count: usize) {
    let mut width = 1;

    for i in 0..triangle_count {
        for row in 0..=i + 1 {
            let spaces = " ".repeat(triangle_count + triangle_count - row - 1);
            let stars = "*".repeat(width + 2 * row);
            println!("{}{}", spaces, stars);
        }
        width += 2;
    }

    let trunk_width = triangle_count;
    let trunk_height = triangle_count / 2 + 1;
    let spaces = " ".repeat(triangle_count - 1);

    for _ in 0..trunk_height {
        println!("{}{}", spaces, "*".repeat(trunk_width));
    }
}

fn main() {
    let triangle_count = 4;
    draw_tree(triangle_count);
}
