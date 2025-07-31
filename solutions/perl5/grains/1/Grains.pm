package Grains 1;
use strict;
use warnings;

sub square {
    my ($n) = @_;

    my $result = 1;
    for (; $n > 1; $n--) {
        $result = $result * 2;
    }

    return $result;
}

sub total {
    my $result = 0;
    for (my $n = 1; $n <= 64; $n++) {
        $result += square($n);
    }

    return $result;
}

1;