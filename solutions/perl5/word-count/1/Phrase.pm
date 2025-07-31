package Phrase;
use strict;
use warnings FATAL => 'all';

sub word_count() {
    my ($phrase) = @_;

    my %result;

    foreach my $word ($phrase =~ /(\w+)/g) {
        $result{lc $word} += 1;
    }

    return \%result;
}

1;