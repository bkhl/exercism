package Leap;
use strict;
use warnings;
use Exporter 'import';
our @EXPORT_OK = qw(is_leap_year);

sub is_leap_year {
    my ($year) = @_;

    return 0 if $year % 100 == 0 and $year % 400 != 0;

    return $year % 4 == 0;
}

1;
