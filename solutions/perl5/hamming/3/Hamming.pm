package Hamming;

use strict;
use warnings FATAL => 'all';

use Exporter 'import';
our @EXPORT_OK = qw(hamming_distance);


sub hamming_distance {
    my ($dna_a, $dna_b) = @_;

    if (length($dna_a) != length($dna_b)) {
        die 'left and right strands must be of equal length';
    }

    return grep { substr($dna_a, $_, 1) ne substr($dna_b, $_, 1) } 0..length($dna_a);
}

1;
