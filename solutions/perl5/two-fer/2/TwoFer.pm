package TwoFer;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(two_fer);

sub two_fer($name) {
    $name ||= 'you';
    return "One for $name, one for me.";
}

1;
