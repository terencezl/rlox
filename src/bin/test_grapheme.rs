use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let complex_source = "let é = '👨‍👩‍👧‍👦';";

    println!("Count: {}", complex_source.graphemes(true).count());
    println!("Grapheme clusters:");
    for (i, grapheme) in complex_source.graphemes(true).enumerate() {
        println!("{}: {}", i, grapheme);
    }
}

// Output:
// Grapheme clusters:
// 1: "l"
// 2: "e"
// 3: "t"
// 4: " "
// 5: "é"
// 6: " "
// 7: "="
// 8: " "
// 9: "'"
// 10: "👨‍👩‍👧‍👦"
// 11: "'"
// 12: ";"
