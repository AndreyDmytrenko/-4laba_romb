fn main() {
    const SIZE: usize = 5; // Розмір ромба (висота верхньої половини)

    let mut result = String::new();

    // Верхня половина ромба
    for i in 0..SIZE {
        for _ in 0..(SIZE - i - 1) {
            result.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            result.push('*');
        }
        result.push('\n');
    }

    // Нижня половина ромба
    for i in (0..SIZE - 1).rev() {
        for _ in 0..(SIZE - i - 1) {
            result.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            result.push('*');
        }
        result.push('\n');
    }

    print!("{}", result); // Виводимо ромб
}
