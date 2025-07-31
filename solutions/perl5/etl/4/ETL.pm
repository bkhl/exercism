package ETL;

use strict;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(score);

use List::Util qw(pairmap);

sub transform() {
    my ($input) = @_;

    return scalar { pairmap { map {  lc $_ => $a } @$b } %$input };
}

1;
