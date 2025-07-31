package Phrase;

use strict;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(word_count);

sub word_count() {
    my ($phrase) = @_;

    my %result;

    $result{lc $_}++ foreach ($phrase =~ /(\w+)/g); 

    return \%result;
}

1;
