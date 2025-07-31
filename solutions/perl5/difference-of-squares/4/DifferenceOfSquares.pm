package DifferenceOfSquares;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(square_of_sum sum_of_squares difference_of_squares);

sub sum_of_squares($n) {
    my $sum = 0;

    $sum += $_ ** 2 for 1 .. $n;

    return $sum;
}

sub square_of_sum($n) {
    my $sum = 0;

    $sum += $_ for 1 .. $n;

    return $sum ** 2;
}

sub difference_of_squares($n) {
    return square_of_sum($n) - sum_of_squares($n);
}

1;
