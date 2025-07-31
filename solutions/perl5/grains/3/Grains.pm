package Grains;

use strict;
use warnings;

use Exporter 'import';
our @EXPORT_OK = qw(grains_on_square total_grains);

use bignum;

sub square {
    my ($n) = @_;

    return 2 ** ($n - 1);
}

sub grains_on_square {
    my ($n) = @_;

    die "square must be between 1 and 64" if $n < 1 or $n > 64;

    return square($n);
}

sub total_grains {
    return square(65) - 1;
}

1;
