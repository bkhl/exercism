package Anagram;
use strict;
use warnings FATAL => 'all';

sub match() {
    my ($word, @candidates) = @_;

    my $word_sorted = sort_word($word);

    my @result;

    foreach my $candidate (@candidates) {
        if ($word ne $candidate && $word_sorted eq sort_word($candidate)) {
            push @result, $candidate;
        }
    }

    return \@result;
}

sub sort_word() {
    my ($word) = @_;

    return join '', sort (split //, lc $word);
}

1;