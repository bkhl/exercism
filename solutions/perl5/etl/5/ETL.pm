package ETL;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(transform);

use List::Util qw(pairmap);

sub transform($input) {
    return scalar { pairmap { map {  lc $_ => $a } @$b } %$input };
}

1;
