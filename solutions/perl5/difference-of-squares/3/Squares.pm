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

    $sum += $_ ** 2 for 1 .. $$self;

    return $sum;
}

sub square_of_sum() {
    my ($self) = @_;

    my $sum = 0;

    $sum += $_ for 1 .. $$self;

    return $sum ** 2;
}


sub difference() {
    my ($self) = @_;

    return $self->square_of_sum - $self->sum_of_squares;
}

1;
