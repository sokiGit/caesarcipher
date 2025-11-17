# Caesar Cipher
A caesar encipher/decipher program. Written in rust. Just learning stuff. Supports special characters.
## Usage
### Encipher
`caesarcipher.exe -r [int] "<string>"`
<br>
Example: `caesarcipher.exe -r 5 "Hello, World!"`
<br>
`-r [int]` Sets the rotation, default is `0`.
<br>
`"<string>"` The text you want to encipher, make sure to put this in quotes if it contains spaces.

### Decipher
`caesarcipher.exe -d -r [int] "<string>"`
<br>
Example: `caesarcipher.exe -d -r 5 "Mjqqt, Btwqi!"`
<br>
`-d` Tells the program to decipher a ciphertext which had the rotation `-r` applied to it.
<br>
`-r [int]` The original rotation used to encipher the original plaintext.
<br>
`"<string>"` The text you want to decipher, make sure to put this in quotes if it contains spaces.