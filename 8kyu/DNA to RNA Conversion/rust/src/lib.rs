fn dna_to_rna(dna: &str) -> String {
    let mut res:Vec<String> = Vec::new();
    for c in dna.chars() {
        if c == 'T' {
            res.push(String::from("U"));
        } else {
            res.push(String::from(c));
        }
    }
    res.join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
      assert_eq!(dna_to_rna("TTTT"), "UUUU");
      assert_eq!(dna_to_rna_best("GCAT"), "GCAU");
      assert_eq!(dna_to_rna("GAAGCTTATCCGTTCCTGAAGGCTGTGGCATCCTCTAAATCAGACTTGGCTACGCCGTTAGCCGAGGGCTTAGCGTTGAGTGTCATTATATACGCGGCCTGCGACCTGGCCACACAATGCCCTCGAAAATTTTTCTTTCGGTTATACGAGTTGCGAAACCTTTCGCGCGTAGACGAAGAATTTGAAGTGGCCTACACCGTTTGGAAAGCCGTTCTCATTAGAATGGTACCGACTACTCGGCTCGGAGTCATTGTATAGGGAGAGTGTCGTATCAACATCACACACTTTTAGCATTTAAGGTCCATGGCCGTTGACAGGTACCGA"), "GAAGCUUAUCCGUUCCUGAAGGCUGUGGCAUCCUCUAAAUCAGACUUGGCUACGCCGUUAGCCGAGGGCUUAGCGUUGAGUGUCAUUAUAUACGCGGCCUGCGACCUGGCCACACAAUGCCCUCGAAAAUUUUUCUUUCGGUUAUACGAGUUGCGAAACCUUUCGCGCGUAGACGAAGAAUUUGAAGUGGCCUACACCGUUUGGAAAGCCGUUCUCAUUAGAAUGGUACCGACUACUCGGCUCGGAGUCAUUGUAUAGGGAGAGUGUCGUAUCAACAUCACACACUUUUAGCAUUUAAGGUCCAUGGCCGUUGACAGGUACCGA");
    }
}

fn dna_to_rna_best(dna: &str) -> String {
    dna.replace("T", "U")
}