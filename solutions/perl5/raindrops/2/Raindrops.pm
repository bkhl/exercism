package Raindrops;

use strict;
use warnings FATAL => 'all';

use Exporter 'import';
our @EXPORT_OK = qw(raindrop);

sub raindrop {
    my ($input) = @_;

    my $raindrops = "";

    $raindrops .= "Pling" if ($input % 3 == 0);
    $raindrops .= "Plang" if ($input % 5 == 0);
    $raindrops .= "Plong" if ($input % 7 == 0);

    return ($raindrops or $input);
}

1;
