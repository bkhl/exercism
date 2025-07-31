package Anagram;

use v5.36;

use warnings FATAL => 'all';

use Exporter 'import';

our @EXPORT_OK = qw(match_anagrams);

sub match_anagrams($word, $candidates) {
    my $lowercase_word = lc $word;

    my $word_sorted = sort_word($lowercase_word);

    my @result;

    foreach my $candidate (@$candidates) {
        my $lowercase_candidate = lc $candidate;
        if ($lowercase_word ne $lowercase_candidate
            && $word_sorted eq sort_word($lowercase_candidate)) {
            push @result, $candidate;
        }
    }

    return \@result;
}

sub sort_word($word) {
    return join '', sort (split //, $word);
}

1;
