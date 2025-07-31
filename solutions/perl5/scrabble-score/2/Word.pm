package Word;
use strict;
use warnings FATAL => 'all';

my %SCORES = (
    'AEIOULNRST' => 1,
    'DG' => 2,
    'BCMP' => 3,
    'FHVWY' => 4,
    'K' => 5,
    'JX' => 8,
    'QZ' => 10,
);

sub new() {
    my $class = shift;

    my ($self) = @_;

    return bless \$self, $class;
}

sub score() {
    my ($self) = @_;

    my $result = 0;

    foreach my $c (split //, uc $$self) {
        foreach my $k (keys %SCORES) {
            if (index($k, $c) != -1) {
                $result += $SCORES{$k};
                last;
            }
        }
    }

    return $result;
}

1;
