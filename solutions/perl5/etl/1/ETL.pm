package ETL;
use strict;
use warnings FATAL => 'all';

sub transform() {
    my ($input) = @_;

    my %output;

    while (my ($score, $letters) = each %$input) {
        foreach my $letter (@$letters) {
            $output{lc $letter} = $score;
        }
    }

    return \%output;
}

1;