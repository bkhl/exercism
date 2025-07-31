package Raindrops;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(raindrop);

use List::Util qw(pairs);

sub raindrop($input) {
    my $raindrops = "";

    foreach my $e (pairs (3 => 'Pling', 5 => 'Plang', 7 => 'Plong')) {
        $raindrops .= $e->value if $input % $e->key == 0;
    }

    return $raindrops || $input;
}

1;
