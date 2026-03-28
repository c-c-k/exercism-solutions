use easybench::bench;

use matching_brackets::brackets_are_balanced;

fn main() {
    let samples = vec![
        "[]",
        "(((185 + 223.85) * 15) - 543)/2",
        "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \\end{array}\\right)",
    ];
    for sample in samples {
        let f = || brackets_are_balanced(sample);
        println!("{sample:?} :\n{}", bench(f))
    }
}
