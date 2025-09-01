# Advanced Perl SScrypt example: Secure file encryption/decryption using AES (Crypt::CBC + Crypt::Cipher::AES)

use strict;
use warnings;
use Crypt::CBC;
use Crypt::Cipher::AES;
use File::Slurp;

sub encrypt_file {
    my ($input, $output, $key) = @_;
    my $cipher = Crypt::CBC->new(
        -key    => $key,
        -cipher => 'Cipher::AES',
        -header => 'none'
    );
    my $data = read_file($input, binmode => ':raw');
    my $encrypted = $cipher->encrypt($data);
    write_file($output, {binmode => ':raw'}, $encrypted);
    print "Plik zaszyfrowany: $output\n";
}

sub decrypt_file {
    my ($input, $output, $key) = @_;
    my $cipher = Crypt::CBC->new(
        -key    => $key,
        -cipher => 'Cipher::AES',
        -header => 'none'
    );
    my $data = read_file($input, binmode => ':raw');
    my $decrypted = $cipher->decrypt($data);
    write_file($output, {binmode => ':raw'}, $decrypted);
    print "Plik odszyfrowany: $output\n";
}

# Przykład użycia:
# encrypt_file('test.txt', 'test.enc', 'superSecretKey123');
# decrypt_file('test.enc', 'test_decrypted.txt', 'superSecretKey123');

1;