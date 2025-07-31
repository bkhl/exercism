package Hamming;
use strict;
use warnings FATAL => 'all';

sub compute() {
    my ($a, $b) = @_;

    if (length($a) ne length($b)) {
        die 'DNA strands must be of equal length';
    }

    my $distance = 0;
    for my $i (0..(length($a) - 1)) {
        if (substr($a, $i, 1) ne substr($b, $i, 1)) {
            $distance += 1;
        }
    }

    return $distance;
}

1;