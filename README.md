# Caesar Cipher

A caesar encipher/decipher program. Written in rust. Just learning stuff. Supports special characters.

## Usage

`caesarcipher.exe [flags] [cipher/plaintext]` (Anything after `cipher/plaintext`is ignored.)

### Help
`caesarcipher.exe -h`, `caesarcipher.exe --help`

```text
Usage:
  -d, --decipher                        Decipher mode (Default behavior is encipher)
  -r [int], --set-rotation [int]        Sets the rotation (default: 0)
  -h, --help                            Print this help message
```

### Encipher

`caesarcipher.exe -r [int] "<string>"`
<br>
Example: `caesarcipher.exe -r 5 "Hello, World!"`
<br>
`-r, --set-rotation [int]` Sets the rotation, default is `0`.
<br>
`"<string>"` The text you want to encipher, make sure to put this in quotes if it contains spaces.

### Decipher

`caesarcipher.exe -d -r [int] "<string>"`
<br>
Example: `caesarcipher.exe -d -r 5 "Mjqqt, Btwqi!"`
<br>
`-d, --decipher` Tells the program to decipher a ciphertext which had the rotation `-r` applied to it.
<br>
`-r, --set-rotation [int]` The original rotation used to encipher the original plaintext.
<br>
`"<string>"` The text you want to decipher, make sure to put this in quotes if it contains spaces.0

## Examples

### Encipher

```text
PS C:\> ./caesarcipher.exe -r 5 "Hello, World!"
Detected original: Hello, World!
Ciphertext: Mjqqt, Btwqi!
```

### Decipher

```text
./caesarcipher.exe -d -r 5 "Mjqqt, Btwqi!"
Detected ciphertext: Mjqqt, Btwqi!
Original: Hello, World!
```