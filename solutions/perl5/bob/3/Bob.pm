package Bob;

use v5.36;
use warnings FATAL => 'all';

use Exporter qw(import);
our @EXPORT_OK = qw(hey);

sub hey($input) {

  if ($input =~ /[A-Z]/ && $input !~ /[a-z]/) {  # yelling
      if ($input =~ /\?\s*$/) {  # yelled question
          return 'Calm down, I know what I\'m doing!';
      } else { # other question
          return 'Whoa, chill out!';
      }
  } elsif ($input =~ /\?\s*$/) {  # question
      return 'Sure.';
  } elsif ($input =~ /^\s*$/ ) { # silence
      return 'Fine. Be that way!';
  } else {
      return 'Whatever.';
  }
}

1;
