rna_to_dna_nucleotide_map = {"G": "C", "C": "G", "T": "A", "A": "U"}


def to_rna(dna_strand):
    return "".join([rna_to_dna_nucleotide_map[c] for c in dna_strand])