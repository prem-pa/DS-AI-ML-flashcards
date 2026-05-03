//! Best-effort LaTeX → Unicode rewriter. Not a TeX engine; just makes the
//! most common interview-card constructs (`\sum`, `\partial`, `\to`, `^2`, ...)
//! readable in a TUI without a math renderer. Anything we don't recognise
//! is left intact.

use regex::Regex;
use std::sync::OnceLock;

const LATEX_TOKENS: &[(&str, &str)] = &[
    // Greek letters
    ("\\alpha", "α"),
    ("\\beta", "β"),
    ("\\gamma", "γ"),
    ("\\delta", "δ"),
    ("\\epsilon", "ε"),
    ("\\varepsilon", "ε"),
    ("\\zeta", "ζ"),
    ("\\eta", "η"),
    ("\\theta", "θ"),
    ("\\vartheta", "ϑ"),
    ("\\iota", "ι"),
    ("\\kappa", "κ"),
    ("\\lambda", "λ"),
    ("\\mu", "μ"),
    ("\\nu", "ν"),
    ("\\xi", "ξ"),
    ("\\pi", "π"),
    ("\\rho", "ρ"),
    ("\\varrho", "ϱ"),
    ("\\sigma", "σ"),
    ("\\varsigma", "ς"),
    ("\\tau", "τ"),
    ("\\upsilon", "υ"),
    ("\\phi", "φ"),
    ("\\varphi", "ϕ"),
    ("\\chi", "χ"),
    ("\\psi", "ψ"),
    ("\\omega", "ω"),
    ("\\Gamma", "Γ"),
    ("\\Delta", "Δ"),
    ("\\Theta", "Θ"),
    ("\\Lambda", "Λ"),
    ("\\Xi", "Ξ"),
    ("\\Pi", "Π"),
    ("\\Sigma", "Σ"),
    ("\\Phi", "Φ"),
    ("\\Psi", "Ψ"),
    ("\\Omega", "Ω"),
    // Operators
    ("\\sum", "∑"),
    ("\\prod", "∏"),
    ("\\int", "∫"),
    ("\\oint", "∮"),
    ("\\partial", "∂"),
    ("\\nabla", "∇"),
    ("\\infty", "∞"),
    ("\\propto", "∝"),
    ("\\forall", "∀"),
    ("\\exists", "∃"),
    ("\\nexists", "∄"),
    ("\\emptyset", "∅"),
    // Arithmetic / set
    ("\\cdot", "·"),
    ("\\cdots", "⋯"),
    ("\\ldots", "…"),
    ("\\dots", "…"),
    ("\\times", "×"),
    ("\\div", "÷"),
    ("\\pm", "±"),
    ("\\mp", "∓"),
    ("\\ast", "∗"),
    ("\\star", "⋆"),
    ("\\circ", "∘"),
    ("\\bullet", "•"),
    ("\\oplus", "⊕"),
    ("\\ominus", "⊖"),
    ("\\otimes", "⊗"),
    ("\\odot", "⊙"),
    // Set theory
    ("\\in", "∈"),
    ("\\notin", "∉"),
    ("\\ni", "∋"),
    ("\\subset", "⊂"),
    ("\\subseteq", "⊆"),
    ("\\supset", "⊃"),
    ("\\supseteq", "⊇"),
    ("\\cup", "∪"),
    ("\\cap", "∩"),
    ("\\setminus", "∖"),
    // Comparison
    ("\\leq", "≤"),
    ("\\le", "≤"),
    ("\\geq", "≥"),
    ("\\ge", "≥"),
    ("\\neq", "≠"),
    ("\\ne", "≠"),
    ("\\equiv", "≡"),
    ("\\approx", "≈"),
    ("\\sim", "∼"),
    ("\\simeq", "≃"),
    ("\\cong", "≅"),
    // Arrows
    ("\\to", "→"),
    ("\\rightarrow", "→"),
    ("\\leftarrow", "←"),
    ("\\Leftarrow", "⇐"),
    ("\\Rightarrow", "⇒"),
    ("\\implies", "⇒"),
    ("\\iff", "⇔"),
    ("\\leftrightarrow", "↔"),
    ("\\mapsto", "↦"),
    ("\\uparrow", "↑"),
    ("\\downarrow", "↓"),
    // Logical
    ("\\land", "∧"),
    ("\\wedge", "∧"),
    ("\\lor", "∨"),
    ("\\vee", "∨"),
    ("\\lnot", "¬"),
    ("\\neg", "¬"),
    // Blackboard bold (common ones)
    ("\\mathbb{R}", "ℝ"),
    ("\\mathbb{N}", "ℕ"),
    ("\\mathbb{Z}", "ℤ"),
    ("\\mathbb{Q}", "ℚ"),
    ("\\mathbb{C}", "ℂ"),
    ("\\mathbb{E}", "𝔼"),
    ("\\mathbb{P}", "ℙ"),
    // Common spacing macros — collapse to single space
    ("\\,", " "),
    ("\\;", " "),
    ("\\:", " "),
    ("\\!", ""),
    ("\\quad", "  "),
    ("\\qquad", "    "),
    ("\\\\", "\n"),
    // Trig / log etc — keep as plain text
    ("\\log", "log"),
    ("\\ln", "ln"),
    ("\\exp", "exp"),
    ("\\sin", "sin"),
    ("\\cos", "cos"),
    ("\\tan", "tan"),
    ("\\arg", "arg"),
    ("\\max", "max"),
    ("\\min", "min"),
    ("\\sup", "sup"),
    ("\\inf", "inf"),
    ("\\arcsin", "arcsin"),
    ("\\arccos", "arccos"),
    ("\\arctan", "arctan"),
    ("\\det", "det"),
    ("\\dim", "dim"),
    ("\\ker", "ker"),
];

/// Apply math substitutions to a prose block. Strips `$...$` and `$$...$$` math
/// delimiters and rewrites known LaTeX commands inside them. Also handles
/// `\frac{a}{b}` → `(a)/(b)`, `\sqrt{x}` → `√(x)`, `^2`/`^3`/`_i` etc.
pub fn prettify_math(text: &str) -> String {
    // 1. Strip `$$ ... $$` then `$ ... $` math fences (keep their contents).
    static DOUBLE_DOLLAR: OnceLock<Regex> = OnceLock::new();
    static SINGLE_DOLLAR: OnceLock<Regex> = OnceLock::new();
    let dd = DOUBLE_DOLLAR.get_or_init(|| Regex::new(r"\$\$([^$]*?)\$\$").unwrap());
    let sd = SINGLE_DOLLAR.get_or_init(|| Regex::new(r"\$([^$]+?)\$").unwrap());

    let s = dd.replace_all(text, "$1");
    let s = sd.replace_all(&s, "$1");

    // 2. \frac{a}{b} → (a)/(b)   (single-level only; nested fractions left alone)
    static FRAC_RE: OnceLock<Regex> = OnceLock::new();
    let frac =
        FRAC_RE.get_or_init(|| Regex::new(r"\\frac\{([^{}]+)\}\{([^{}]+)\}").unwrap());
    let s = frac.replace_all(&s, "($1)/($2)").to_string();

    // 3. \sqrt{x} → √(x) and \sqrt[n]{x} → ⁿ√(x)  (rough)
    static SQRT_RE: OnceLock<Regex> = OnceLock::new();
    let sqrt = SQRT_RE.get_or_init(|| Regex::new(r"\\sqrt\{([^{}]+)\}").unwrap());
    let s = sqrt.replace_all(&s, "√($1)").to_string();

    // 4. \hat{x} → x̂ (combining accent — okay in monospace)
    static HAT_RE: OnceLock<Regex> = OnceLock::new();
    let hat = HAT_RE.get_or_init(|| Regex::new(r"\\hat\{([^{}]+)\}").unwrap());
    let s = hat.replace_all(&s, "$1\u{0302}").to_string();
    static BAR_RE: OnceLock<Regex> = OnceLock::new();
    let bar = BAR_RE.get_or_init(|| Regex::new(r"\\bar\{([^{}]+)\}").unwrap());
    let s = bar.replace_all(&s, "$1\u{0304}").to_string();
    static TILDE_RE: OnceLock<Regex> = OnceLock::new();
    let tilde = TILDE_RE.get_or_init(|| Regex::new(r"\\tilde\{([^{}]+)\}").unwrap());
    let s = tilde.replace_all(&s, "$1\u{0303}").to_string();

    // 5. {x} unwrap when it's a single-token group right after _ or ^ — handled
    //    by the super/subscript pass below.

    // 6. Token table substitutions. Sort longest-first to avoid prefix collisions
    //    (e.g. \mathbb{R} before \math, \Rightarrow before \rightarrow).
    let mut tokens: Vec<&(&str, &str)> = LATEX_TOKENS.iter().collect();
    tokens.sort_by_key(|&&(k, _)| std::cmp::Reverse(k.len()));
    let mut s = s;
    for (k, v) in tokens {
        if s.contains(k) {
            s = s.replace(k, v);
        }
    }

    // 7. Superscripts: ^2 ^3 ^n ^{ab}. We only transform single-char and short
    //    runs; bail to plain ^... otherwise.
    s = pretty_super_sub(&s);

    s
}

fn pretty_super_sub(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.char_indices().peekable();
    while let Some((i, ch)) = chars.next() {
        if ch == '^' || ch == '_' {
            let map = if ch == '^' { sup_char } else { sub_char };
            match chars.peek().copied() {
                Some((j, '{')) => {
                    if let Some(close) = find_matching_brace(s, j) {
                        let inner = &s[j + 1..close];
                        if let Some(translit) = translate_str(inner, map) {
                            out.push_str(&translit);
                        } else {
                            out.push(ch);
                            out.push('(');
                            out.push_str(inner);
                            out.push(')');
                        }
                        // skip everything up to and including the closing brace
                        while let Some((k, _)) = chars.peek().copied() {
                            if k > close {
                                break;
                            }
                            chars.next();
                        }
                        continue;
                    }
                }
                Some((_, nxt)) => {
                    if let Some(c) = map(nxt) {
                        out.push(c);
                        chars.next();
                        continue;
                    }
                }
                None => {}
            }
        }
        // default: emit the original char
        let _ = i;
        out.push(ch);
    }
    out
}

fn find_matching_brace(s: &str, open: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    if bytes.get(open) != Some(&b'{') {
        return None;
    }
    let mut depth = 1i32;
    let mut i = open + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn translate_str(s: &str, map: fn(char) -> Option<char>) -> Option<String> {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match map(ch) {
            Some(c) => out.push(c),
            None if ch == ' ' => out.push(' '),
            None => return None,
        }
    }
    Some(out)
}

fn sup_char(c: char) -> Option<char> {
    Some(match c {
        '0' => '⁰', '1' => '¹', '2' => '²', '3' => '³', '4' => '⁴',
        '5' => '⁵', '6' => '⁶', '7' => '⁷', '8' => '⁸', '9' => '⁹',
        '+' => '⁺', '-' => '⁻', '=' => '⁼', '(' => '⁽', ')' => '⁾',
        'n' => 'ⁿ', 'i' => 'ⁱ', 'T' => 'ᵀ',
        _ => return None,
    })
}

fn sub_char(c: char) -> Option<char> {
    Some(match c {
        '0' => '₀', '1' => '₁', '2' => '₂', '3' => '₃', '4' => '₄',
        '5' => '₅', '6' => '₆', '7' => '₇', '8' => '₈', '9' => '₉',
        '+' => '₊', '-' => '₋', '=' => '₌', '(' => '₍', ')' => '₎',
        'a' => 'ₐ', 'e' => 'ₑ', 'i' => 'ᵢ', 'j' => 'ⱼ', 'k' => 'ₖ',
        'l' => 'ₗ', 'm' => 'ₘ', 'n' => 'ₙ', 'o' => 'ₒ', 'p' => 'ₚ',
        'r' => 'ᵣ', 's' => 'ₛ', 't' => 'ₜ', 'u' => 'ᵤ', 'v' => 'ᵥ',
        'x' => 'ₓ',
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greek_and_operators() {
        let s = prettify_math(r"$\sum_{i=1}^{n} \alpha_i \cdot x_i$");
        assert!(s.contains('∑'), "got: {s}");
        assert!(s.contains('α'));
        assert!(s.contains('·'));
        assert!(s.contains('ᵢ'));
        assert!(!s.contains('$'));
    }

    #[test]
    fn frac_and_sqrt() {
        let s = prettify_math(r"$\frac{1}{n} \sum x_i$, $\sqrt{n}$");
        assert!(s.contains("(1)/(n)"));
        assert!(s.contains('√'));
    }

    #[test]
    fn arrows_and_implications() {
        let s = prettify_math(r"a \to b \implies c \iff d");
        assert!(s.contains('→'));
        assert!(s.contains('⇒'));
        assert!(s.contains('⇔'));
    }

    #[test]
    fn unknown_tokens_are_preserved() {
        let s = prettify_math(r"\unknown_macro stays");
        assert!(s.contains("\\unknown"));
    }
}
