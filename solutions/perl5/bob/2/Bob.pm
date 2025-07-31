package Bob;
use strict;
use warnings;
use Exporter 'import';
our @EXPORT_OK = qw(hey);

sub hey {
  my ($input) = @_;

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
