package Squares;
use strict;
use warnings FATAL => 'all';

sub new() {
    my $class = shift;
    my ($number) = @_;

    return bless \$number, $class;
}

sub sum_of_squares() {
    my ($self) = @_;

    my $sum = 0;

    my $i = $$self;
    while ($i > 0) {
        $sum += $i-- ** 2;
    }

    return $sum;
}

sub square_of_sum() {
    my ($self) = @_;

    my $sum = 0;

    my $i = $$self;
    while ($i > 0) {
        $sum += $i--;
    }

    return $sum ** 2;
}


sub difference() {
    my ($self) = @_;

    return $self->square_of_sum - $self->sum_of_squares;
}

1;
