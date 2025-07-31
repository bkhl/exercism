# Declare package 'Leap' with version
package Leap 2;
use strict;
use warnings;

sub is_leap {
    my ($year) = @_;
    if ($year % 100 == 0 && $year % 400 != 0) {
        return 0;
    } elsif ($year % 4 == 0) {
        return 1;
    } else {
        return 0;
    }
}

1;