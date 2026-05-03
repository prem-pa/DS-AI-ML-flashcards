fn main() {
    let inputs = [
        r"$\sum_{i=1}^{n} \alpha_i \cdot x_i$",
        r"\frac{1}{n} \sum (x_i - \bar{x})^2",
        r"f: \mathbb{R}^n \to \mathbb{R}",
        r"\partial L / \partial \theta",
        r"\mathcal{O}(n \log n) \implies polynomial",
        r"a \leq b \neq c \approx d",
    ];
    for s in inputs {
        let p = flashcards::render::prettify_math(s);
        println!("  {}", p);
    }
}
