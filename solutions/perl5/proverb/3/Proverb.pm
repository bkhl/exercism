package Proverb;
use strict;
use warnings FATAL => 'all';

sub proverb() {
    my ($words_ref, $qualifier) = @_;

    my @words = @$words_ref;
    my ($first_word) = @words;
    my $proverb = '';

    while (@words != 1 && (my $word = shift @words)) {
        $proverb .= sprintf
            "For want of a %s the %s was lost.\n",
            $word, $words[0];
    }

    $proverb .= sprintf
        "And all for the want of a %s.",
        join ' ', grep { $_ } $qualifier, $first_word;


    return $proverb;
}

1;
