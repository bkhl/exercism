package Grains 1;

use strict;
use warnings;

use bignum;

sub square {
    my ($n) = @_;

    return 2 ** ($n - 1);
}

sub total {
    return square(65) - 1;
}

1;