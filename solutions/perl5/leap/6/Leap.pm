package Leap;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(is_leap_year);

sub is_leap_year($year) {
    return 0 if $year % 100 == 0 and $year % 400 != 0;
    return $year % 4 == 0;
}

1;
