package HelloWorld 1;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(hello);

sub hello {
    return "Hello, World!";
}

1;
