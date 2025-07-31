package Hamming;

use strict;
use warnings FATAL => 'all';

use Exporter 'import';
our @EXPORT_OK = qw(hamming_distance);


sub hamming_distance {
    my ($a, $b) = @_;

    if (length($a) != length($b)) {
        die 'left and right strands must be of equal length';
    }

    my $distance = 0;
    for my $i (0..(length($a) - 1)) {
        $distance += 1 if substr($a, $i, 1) ne substr($b, $i, 1);
    }

    return $distance;
}

1;
