package ScrabbleScore;
use strict;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(score);

use List::Util qw(pairmap sum0);

my %SCORES = pairmap { map { $_ => $b } split '', $a } (
    'AEIOULNRST' => 1,
    'DG' => 2,
    'BCMP' => 3,
    'FHVWY' => 4,
    'K' => 5,
    'JX' => 8,
    'QZ' => 10,
);

sub score {
    my ($input) = @_;

    return sum0 map { $SCORES{$_} } split '', uc $input;
}

1;
