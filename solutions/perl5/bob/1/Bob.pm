# Declare package 'Bob' with version
package Bob 2;
use strict;
use warnings;

sub hey {
  my ($msg) = @_;
  if ($msg =~ /[A-Z]/ && $msg !~ /[a-z]/) {
      return 'Whoa, chill out!';
  } elsif ($msg =~ /\?\s*$/) {
      return 'Sure.';
  } elsif ($msg =~ /^\s*$/ ) {
      return 'Fine. Be that way!';
  } else {
      return 'Whatever.';
  }
}

1;