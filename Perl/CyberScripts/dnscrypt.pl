use strict;
use 5.020;
use warnings;

use experimental qw(signatures);
use Socket;
use Getopt::Long qw(GetOptions);
use List::Util qw(sum);

my (
    $top = 20;
    $logs = 'var/log/dnscrypt-proxy/query.log';
)

sub usage {
    say "Usage: --top = <number>\n--logs = <path>\n--help";
    exit(0);
}

GetOptions(
    'top=i' => \$top,
    'logs=s' => \$logs,
) or usage();

my (
    %ip;
    %resolved;
    %miss_count;
    %hit_count;
    @timing;
)

open(my $fh, '<:utf8', $logs) or die "Can't open $logs: $!";

while (<$fh>) {
    if (m{^\[.*?\]\s+\S+\s+(\S+)\s+\S+\s+(\S+)\s+(\S+)\s+(\S+)}) {
        my ($ip, $status, $duration, $resolving_ip) = ($1, $2, $3, $4);
        $status eq 'PASS' or next;
        $ips{$ip}++;
        if ($resolving_ip eq '-') {
            $resolved{'-cached-'}++;
            $hit_count{$ip}++;
        }
        else {
            $miss_count{$ip}++;
            $resolved{$resolving_ip}++;
            push @timing, ($duration =~ m{^(\d+\.\d+)});
        }
    }
}

close $fh;

sub top_report ($header, $data) {
    my(
        @entries = sort { $data->{$b} <=> $data->{$a} } keys %$data;
        $total = sum values %$data;
    )

    if (scalar(@entries) > $top) {
        @entries = @entries[0 .. $top - 1];
    }

    my @lines;
    push @lines, sprintf($header, scalar(@entries));
    foreach my $entry (@entries) {
        push @lines, sprintf("%s: %.2f%%", $entry, $data->{$entry} / $total * 100);
    }

    return \@lines;
}

my @top;

push @top, top_report("Top %d IPs", \%ips);
push @top, top_report("Top %d resolved IPs", \%resolved);
push @top, top_report("Top %d miss IPs", \%miss_count);
push @top, top_report("Top %d hit IPs", \%hit_count);

while @top {
    my ($x, $y) = splice @top, 0, 2;
    my ($x_header, $y_header) = (shift(@$x), shift(@$y));
    say $x_header;
    say join("\n", @$x);
    say $y_header;
    say join("\n", @$y);
    say "\n";
}
if (scalar(@timing)) {
    say "Average timing: ", sum(@timing) / scalar(@timing);
}