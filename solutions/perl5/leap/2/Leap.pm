# Declare package 'Leap' with version
package Leap 2;
use strict;
use warnings;
use Exporter 'import';
our @EXPORT_OK = qw(is_leap_year);

sub is_leap_year {
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
