package Raindrops;

use strict;
use warnings FATAL => 'all';

use Exporter 'import';
our @EXPORT_OK = qw(raindrop);

use List::Util qw(pairs);

sub raindrop {
    my ($input) = @_;

    my $raindrops = "";

    foreach my $e (pairs (3 => 'Pling', 5 => 'Plang', 7 => 'Plong')) {
        $raindrops .= $e->value if $input % $e->key == 0;
    }

    return $raindrops || $input;
}

1;
