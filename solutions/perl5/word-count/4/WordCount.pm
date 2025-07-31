package WordCount;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(count_words);

sub count_words($phrase) {
    my %result;

    $result{lc $_}++ foreach ($phrase =~ /(\w+(?:'\w+)?)/g);

    return \%result;
}

1;
