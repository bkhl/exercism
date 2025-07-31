package Grains;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(grains_on_square total_grains);

use bignum;

sub square($n) {
    return 2 ** ($n - 1);
}

sub grains_on_square($n) {
    die "square must be between 1 and 64" if $n < 1 or $n > 64;
    return square($n);
}

sub total_grains() {
    return square(65) - 1;
}

1;
