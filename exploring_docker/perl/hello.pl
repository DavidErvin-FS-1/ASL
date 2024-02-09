#!/usr/bin/perl

use strict;
use warnings;
use POSIX qw/strftime/;

print "Hello, World!\n";

my $date = strftime "%Y-%m-%d", localtime;

print "$date\n";
