package Hamming;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(hamming_distance);

sub hamming_distance($dna_a, $dna_b) {
    if (length($dna_a) != length($dna_b)) {
        die 'left and right strands must be of equal length';
    }
    return grep { substr($dna_a, $_, 1) ne substr($dna_b, $_, 1) } 0..length($dna_a);
}

1;
