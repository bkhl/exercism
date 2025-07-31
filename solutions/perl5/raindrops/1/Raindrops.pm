package Raindrops;
use strict;
use warnings FATAL => 'all';

sub convert() {
    my ($input) = @_;

    my $number = int($input);
    my $raindrops = "";

    if ($number % 3 == 0) {$raindrops .= "Pling";}
    if ($number % 5 == 0) {$raindrops .= "Plang";}
    if ($number % 7 == 0) {$raindrops .= "Plong";}

    if ($raindrops) {
        return $raindrops;
    } else {
        return $number;
    }
}

1;