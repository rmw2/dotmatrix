/// Bit patterns for the CP437 font for 8x8 dot matrix
/// which covers all 256 ascii characters.
/// Each array element consists of the 8 bytes used to build up an image
/// NOTE: If you are looking at the chip and the part number is on the top then you are actually
/// looking at the chip upside down and value 1 at row 0 is the led in the bottom left corner.
/// The font below has been rotated 90 degrees from the font in the forum post (linked see lib.rs)
pub const CP437FONT: [[u8; 8]; 256] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 0x00
    [0x7E, 0x81, 0x99, 0xBD, 0x81, 0xA5, 0x81, 0x7E], // 0x01
    [0x7E, 0xFF, 0xE7, 0xC3, 0xFF, 0xDB, 0xFF, 0x7E], // 0x02
    [0x00, 0x08, 0x1C, 0x3E, 0x7F, 0x7F, 0x7F, 0x36], // 0x03
    [0x00, 0x08, 0x1C, 0x3E, 0x7F, 0x3E, 0x1C, 0x08], // 0x04
    [0x3E, 0x1C, 0x3E, 0x7F, 0x7F, 0x1C, 0x3E, 0x1C], // 0x05
    [0x3E, 0x1C, 0x3E, 0x7F, 0x3E, 0x1C, 0x08, 0x08], // 0x06
    [0x00, 0x00, 0x18, 0x3C, 0x3C, 0x18, 0x00, 0x00], // 0x07
    [0xFF, 0xFF, 0xE7, 0xC3, 0xC3, 0xE7, 0xFF, 0xFF], // 0x08
    [0x00, 0x3C, 0x66, 0x42, 0x42, 0x66, 0x3C, 0x00], // 0x09
    [0xFF, 0xC3, 0x99, 0xBD, 0xBD, 0x99, 0xC3, 0xFF], // 0x0A
    [0x1E, 0x33, 0x33, 0x33, 0xBE, 0xF0, 0xE0, 0xF0], // 0x0B
    [0x18, 0x7E, 0x18, 0x3C, 0x66, 0x66, 0x66, 0x3C], // 0x0C
    [0x07, 0x0F, 0x0E, 0x0C, 0x0C, 0xFC, 0xCC, 0xFC], // 0x0D
    [0x03, 0x67, 0xE6, 0xC6, 0xC6, 0xFE, 0xC6, 0xFE], // 0x0E
    [0x99, 0x5A, 0x3C, 0xE7, 0xE7, 0x3C, 0x5A, 0x99], // 0x0F
    [0x00, 0x01, 0x07, 0x1F, 0x7F, 0x1F, 0x07, 0x01], // 0x10
    [0x00, 0x40, 0x70, 0x7C, 0x7F, 0x7C, 0x70, 0x40], // 0x11
    [0x18, 0x3C, 0x7E, 0x18, 0x18, 0x7E, 0x3C, 0x18], // 0x12
    [0x00, 0x66, 0x00, 0x66, 0x66, 0x66, 0x66, 0x66], // 0x13
    [0x00, 0xD8, 0xD8, 0xD8, 0xDE, 0xDB, 0xDB, 0xFE], // 0x14
    [0x1E, 0x33, 0x1C, 0x36, 0x36, 0x1C, 0xC6, 0x7C], // 0x15
    [0x00, 0x7E, 0x7E, 0x7E, 0x00, 0x00, 0x00, 0x00], // 0x16
    [0xFF, 0x18, 0x3C, 0x7E, 0x18, 0x7E, 0x3C, 0x18], // 0x17
    [0x00, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x3C, 0x18], // 0x18
    [0x00, 0x18, 0x3C, 0x7E, 0x18, 0x18, 0x18, 0x18], // 0x19
    [0x00, 0x00, 0x18, 0x30, 0x7F, 0x30, 0x18, 0x00], // 0x1A
    [0x00, 0x00, 0x0C, 0x06, 0x7F, 0x06, 0x0C, 0x00], // 0x1B
    [0x00, 0x00, 0x7F, 0x03, 0x03, 0x03, 0x00, 0x00], // 0x1C
    [0x00, 0x00, 0x24, 0x66, 0xFF, 0x66, 0x24, 0x00], // 0x1D
    [0x00, 0x00, 0xFF, 0xFF, 0x7E, 0x3C, 0x18, 0x00], // 0x1E
    [0x00, 0x00, 0x18, 0x3C, 0x7E, 0xFF, 0xFF, 0x00], // 0x1F
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ' '
    [0x00, 0x0C, 0x00, 0x0C, 0x0C, 0x1E, 0x1E, 0x0C], // '!'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x36, 0x36], // '"'
    [0x00, 0x36, 0x36, 0x7F, 0x36, 0x7F, 0x36, 0x36], // '#'
    [0x00, 0x0C, 0x1F, 0x30, 0x1E, 0x03, 0x3E, 0x0C], // '$'
    [0x00, 0x63, 0x66, 0x0C, 0x18, 0x33, 0x63, 0x00], // '%'
    [0x00, 0x6E, 0x33, 0x3B, 0x6E, 0x1C, 0x36, 0x1C], // '&'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x06, 0x06], // '''
    [0x00, 0x18, 0x0C, 0x06, 0x06, 0x06, 0x0C, 0x18], // '('
    [0x00, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x0C, 0x06], // ')'
    [0x00, 0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00], // '*'
    [0x00, 0x00, 0x0C, 0x0C, 0x3F, 0x0C, 0x0C, 0x00], // '+'
    [0x06, 0x0C, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00], // ','
    [0x00, 0x00, 0x00, 0x00, 0x3F, 0x00, 0x00, 0x00], // '-'
    [0x00, 0x0C, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00], // '.'
    [0x00, 0x01, 0x03, 0x06, 0x0C, 0x18, 0x30, 0x60], // '/'
    [0x00, 0x3E, 0x67, 0x6F, 0x7B, 0x73, 0x63, 0x3E], // '0'
    [0x00, 0x3F, 0x0C, 0x0C, 0x0C, 0x0C, 0x0E, 0x0C], // '1'
    [0x00, 0x3F, 0x33, 0x06, 0x1C, 0x30, 0x33, 0x1E], // '2'
    [0x00, 0x1E, 0x33, 0x30, 0x1C, 0x30, 0x33, 0x1E], // '3'
    [0x00, 0x78, 0x30, 0x7F, 0x33, 0x36, 0x3C, 0x38], // '4'
    [0x00, 0x1E, 0x33, 0x30, 0x30, 0x1F, 0x03, 0x3F], // '5'
    [0x00, 0x1E, 0x33, 0x33, 0x1F, 0x03, 0x06, 0x1C], // '6'
    [0x00, 0x0C, 0x0C, 0x0C, 0x18, 0x30, 0x33, 0x3F], // '7'
    [0x00, 0x1E, 0x33, 0x33, 0x1E, 0x33, 0x33, 0x1E], // '8'
    [0x00, 0x0E, 0x18, 0x30, 0x3E, 0x33, 0x33, 0x1E], // '9'
    [0x00, 0x0C, 0x0C, 0x00, 0x00, 0x0C, 0x0C, 0x00], // ':'
    [0x06, 0x0C, 0x0C, 0x00, 0x00, 0x0C, 0x0C, 0x00], // ';'
    [0x00, 0x18, 0x0C, 0x06, 0x03, 0x06, 0x0C, 0x18], // '<'
    [0x00, 0x00, 0x3F, 0x00, 0x00, 0x3F, 0x00, 0x00], // '='
    [0x00, 0x06, 0x0C, 0x18, 0x30, 0x18, 0x0C, 0x06], // '>'
    [0x00, 0x0C, 0x00, 0x0C, 0x18, 0x30, 0x33, 0x1E], // '?'
    [0x00, 0x1E, 0x03, 0x7B, 0x7B, 0x7B, 0x63, 0x3E], // '@'
    [0x00, 0x33, 0x33, 0x3F, 0x33, 0x33, 0x1E, 0x0C], // 'A'
    [0x00, 0x3F, 0x66, 0x66, 0x3E, 0x66, 0x66, 0x3F], // 'B'
    [0x00, 0x3C, 0x66, 0x03, 0x03, 0x03, 0x66, 0x3C], // 'C'
    [0x00, 0x1F, 0x36, 0x66, 0x66, 0x66, 0x36, 0x1F], // 'D'
    [0x00, 0x7F, 0x46, 0x16, 0x1E, 0x16, 0x46, 0x7F], // 'E'
    [0x00, 0x0F, 0x06, 0x16, 0x1E, 0x16, 0x46, 0x7F], // 'F'
    [0x00, 0x7C, 0x66, 0x73, 0x03, 0x03, 0x66, 0x3C], // 'G'
    [0x00, 0x33, 0x33, 0x33, 0x3F, 0x33, 0x33, 0x33], // 'H'
    [0x00, 0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x1E], // 'I'
    [0x00, 0x1E, 0x33, 0x33, 0x30, 0x30, 0x30, 0x78], // 'J'
    [0x00, 0x67, 0x66, 0x36, 0x1E, 0x36, 0x66, 0x67], // 'K'
    [0x00, 0x7F, 0x66, 0x46, 0x06, 0x06, 0x06, 0x0F], // 'L'
    [0x00, 0x63, 0x63, 0x6B, 0x7F, 0x7F, 0x77, 0x63], // 'M'
    [0x00, 0x63, 0x63, 0x73, 0x7B, 0x6F, 0x67, 0x63], // 'N'
    [0x00, 0x1C, 0x36, 0x63, 0x63, 0x63, 0x36, 0x1C], // 'O'
    [0x00, 0x0F, 0x06, 0x06, 0x3E, 0x66, 0x66, 0x3F], // 'P'
    [0x00, 0x38, 0x1E, 0x3B, 0x33, 0x33, 0x33, 0x1E], // 'Q'
    [0x00, 0x67, 0x66, 0x36, 0x3E, 0x66, 0x66, 0x3F], // 'R'
    [0x00, 0x1E, 0x33, 0x38, 0x0E, 0x07, 0x33, 0x1E], // 'S'
    [0x00, 0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x2D, 0x3F], // 'T'
    [0x00, 0x3F, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33], // 'U'
    [0x00, 0x0C, 0x1E, 0x33, 0x33, 0x33, 0x33, 0x33], // 'V'
    [0x00, 0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63], // 'W'
    [0x00, 0x63, 0x36, 0x1C, 0x1C, 0x36, 0x63, 0x63], // 'X'
    [0x00, 0x1E, 0x0C, 0x0C, 0x1E, 0x33, 0x33, 0x33], // 'Y'
    [0x00, 0x7F, 0x66, 0x4C, 0x18, 0x31, 0x63, 0x7F], // 'Z'
    [0x00, 0x1E, 0x06, 0x06, 0x06, 0x06, 0x06, 0x1E], // '['
    [0x00, 0x40, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x03], // backslash
    [0x00, 0x1E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x1E], // ']'
    [0x00, 0x00, 0x00, 0x00, 0x63, 0x36, 0x1C, 0x08], // '^'
    [0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // '_'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x0C, 0x0C], // '`'
    [0x00, 0x6E, 0x33, 0x3E, 0x30, 0x1E, 0x00, 0x00], // 'a'
    [0x00, 0x3B, 0x66, 0x66, 0x3E, 0x06, 0x06, 0x07], // 'b'
    [0x00, 0x1E, 0x33, 0x03, 0x33, 0x1E, 0x00, 0x00], // 'c'
    [0x00, 0x6E, 0x33, 0x33, 0x3E, 0x30, 0x30, 0x38], // 'd'
    [0x00, 0x1E, 0x03, 0x3F, 0x33, 0x1E, 0x00, 0x00], // 'e'
    [0x00, 0x0F, 0x06, 0x06, 0x0F, 0x06, 0x36, 0x1C], // 'f'
    [0x1F, 0x30, 0x3E, 0x33, 0x33, 0x6E, 0x00, 0x00], // 'g'
    [0x00, 0x67, 0x66, 0x66, 0x6E, 0x36, 0x06, 0x07], // 'h'
    [0x00, 0x1E, 0x0C, 0x0C, 0x0C, 0x0E, 0x00, 0x0C], // 'i'
    [0x1E, 0x33, 0x33, 0x30, 0x30, 0x30, 0x00, 0x30], // 'j'
    [0x00, 0x67, 0x36, 0x1E, 0x36, 0x66, 0x06, 0x07], // 'k'
    [0x00, 0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0E], // 'l'
    [0x00, 0x63, 0x6B, 0x7F, 0x7F, 0x33, 0x00, 0x00], // 'm'
    [0x00, 0x33, 0x33, 0x33, 0x33, 0x1F, 0x00, 0x00], // 'n'
    [0x00, 0x1E, 0x33, 0x33, 0x33, 0x1E, 0x00, 0x00], // 'o'
    [0x0F, 0x06, 0x3E, 0x66, 0x66, 0x3B, 0x00, 0x00], // 'p'
    [0x78, 0x30, 0x3E, 0x33, 0x33, 0x6E, 0x00, 0x00], // 'q'
    [0x00, 0x0F, 0x06, 0x66, 0x6E, 0x3B, 0x00, 0x00], // 'r'
    [0x00, 0x1F, 0x30, 0x1E, 0x03, 0x3E, 0x00, 0x00], // 's'
    [0x00, 0x18, 0x2C, 0x0C, 0x0C, 0x3E, 0x0C, 0x08], // 't'
    [0x00, 0x6E, 0x33, 0x33, 0x33, 0x33, 0x00, 0x00], // 'u'
    [0x00, 0x0C, 0x1E, 0x33, 0x33, 0x33, 0x00, 0x00], // 'v'
    [0x00, 0x36, 0x7F, 0x7F, 0x6B, 0x63, 0x00, 0x00], // 'w'
    [0x00, 0x63, 0x36, 0x1C, 0x36, 0x63, 0x00, 0x00], // 'x'
    [0x1F, 0x30, 0x3E, 0x33, 0x33, 0x33, 0x00, 0x00], // 'y'
    [0x00, 0x3F, 0x26, 0x0C, 0x19, 0x3F, 0x00, 0x00], // 'z'
    [0x00, 0x38, 0x0C, 0x0C, 0x07, 0x0C, 0x0C, 0x38], // '['
    [0x00, 0x18, 0x18, 0x18, 0x00, 0x18, 0x18, 0x18], // '|'
    [0x00, 0x07, 0x0C, 0x0C, 0x38, 0x0C, 0x0C, 0x07], // ']'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3B, 0x6E], // '~'
    [0x00, 0x7F, 0x63, 0x63, 0x36, 0x1C, 0x08, 0x00], // 0x7F
    [0x1E, 0x30, 0x18, 0x1E, 0x33, 0x03, 0x33, 0x1E], // 0x80
    [0x00, 0x7E, 0x33, 0x33, 0x33, 0x00, 0x33, 0x00], // 0x81
    [0x00, 0x1E, 0x03, 0x3F, 0x33, 0x1E, 0x00, 0x38], // 0x82
    [0x00, 0xFC, 0x66, 0x7C, 0x60, 0x3C, 0xC3, 0x7E], // 0x83
    [0x00, 0x7E, 0x33, 0x3E, 0x30, 0x1E, 0x00, 0x33], // 0x84
    [0x00, 0x7E, 0x33, 0x3E, 0x30, 0x1E, 0x00, 0x07], // 0x85
    [0x00, 0x7E, 0x33, 0x3E, 0x30, 0x1E, 0x0C, 0x0C], // 0x86
    [0x1C, 0x30, 0x1E, 0x03, 0x03, 0x1E, 0x00, 0x00], // 0x87
    [0x00, 0x3C, 0x06, 0x7E, 0x66, 0x3C, 0xC3, 0x7E], // 0x88
    [0x00, 0x1E, 0x03, 0x3F, 0x33, 0x1E, 0x00, 0x33], // 0x89
    [0x00, 0x1E, 0x03, 0x3F, 0x33, 0x1E, 0x00, 0x07], // 0x8A
    [0x00, 0x1E, 0x0C, 0x0C, 0x0C, 0x0E, 0x00, 0x33], // 0x8B
    [0x00, 0x3C, 0x18, 0x18, 0x18, 0x1C, 0x63, 0x3E], // 0x8C
    [0x00, 0x1E, 0x0C, 0x0C, 0x0C, 0x0E, 0x00, 0x07], // 0x8D
    [0x00, 0x63, 0x63, 0x7F, 0x63, 0x36, 0x1C, 0x63], // 0x8E
    [0x00, 0x33, 0x3F, 0x33, 0x1E, 0x00, 0x0C, 0x0C], // 0x8F
    [0x00, 0x3F, 0x06, 0x1E, 0x06, 0x3F, 0x00, 0x38], // 0x90
    [0x00, 0xFE, 0x33, 0xFE, 0x30, 0xFE, 0x00, 0x00], // 0x91
    [0x00, 0x73, 0x33, 0x33, 0x7F, 0x33, 0x36, 0x7C], // 0x92
    [0x00, 0x1E, 0x33, 0x33, 0x1E, 0x00, 0x33, 0x1E], // 0x93
    [0x00, 0x1E, 0x33, 0x33, 0x1E, 0x00, 0x33, 0x00], // 0x94
    [0x00, 0x1E, 0x33, 0x33, 0x1E, 0x00, 0x07, 0x00], // 0x95
    [0x00, 0x7E, 0x33, 0x33, 0x33, 0x00, 0x33, 0x1E], // 0x96
    [0x00, 0x7E, 0x33, 0x33, 0x33, 0x00, 0x07, 0x00], // 0x97
    [0x1F, 0x30, 0x3E, 0x33, 0x33, 0x00, 0x33, 0x00], // 0x98
    [0x00, 0x18, 0x3C, 0x66, 0x66, 0x3C, 0x18, 0xC3], // 0x99
    [0x00, 0x1E, 0x33, 0x33, 0x33, 0x33, 0x00, 0x33], // 0x9A
    [0x18, 0x18, 0x7E, 0x03, 0x03, 0x7E, 0x18, 0x18], // 0x9B
    [0x00, 0x3F, 0x67, 0x06, 0x0F, 0x26, 0x36, 0x1C], // 0x9C
    [0x0C, 0x0C, 0x3F, 0x0C, 0x3F, 0x1E, 0x33, 0x33], // 0x9D
    [0xE3, 0x63, 0xF3, 0x63, 0x5F, 0x33, 0x33, 0x1F], // 0x9E
    [0x0E, 0x1B, 0x18, 0x18, 0x3C, 0x18, 0xD8, 0x70], // 0x9F
    [0x00, 0x7E, 0x33, 0x3E, 0x30, 0x1E, 0x00, 0x38], // 0xA0
    [0x00, 0x1E, 0x0C, 0x0C, 0x0C, 0x0E, 0x00, 0x1C], // 0xA1
    [0x00, 0x1E, 0x33, 0x33, 0x1E, 0x00, 0x38, 0x00], // 0xA2
    [0x00, 0x7E, 0x33, 0x33, 0x33, 0x00, 0x38, 0x00], // 0xA3
    [0x00, 0x33, 0x33, 0x33, 0x1F, 0x00, 0x1F, 0x00], // 0xA4
    [0x00, 0x33, 0x3B, 0x3F, 0x37, 0x33, 0x00, 0x3F], // 0xA5
    [0x00, 0x00, 0x7E, 0x00, 0x7C, 0x36, 0x36, 0x3C], // 0xA6
    [0x00, 0x00, 0x3E, 0x00, 0x1C, 0x36, 0x36, 0x1C], // 0xA7
    [0x00, 0x1E, 0x33, 0x03, 0x06, 0x0C, 0x00, 0x0C], // 0xA8
    [0x00, 0x00, 0x03, 0x03, 0x3F, 0x00, 0x00, 0x00], // 0xA9
    [0x00, 0x00, 0x30, 0x30, 0x3F, 0x00, 0x00, 0x00], // 0xAA
    [0xF0, 0x33, 0x66, 0xCC, 0x7B, 0x33, 0x63, 0xC3], // 0xAB
    [0xC0, 0xF3, 0xF6, 0xEC, 0xDB, 0x33, 0x63, 0xC3], // 0xAC
    [0x00, 0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x18], // 0xAD
    [0x00, 0x00, 0xCC, 0x66, 0x33, 0x66, 0xCC, 0x00], // 0xAE
    [0x00, 0x00, 0x33, 0x66, 0xCC, 0x66, 0x33, 0x00], // 0xAF
    [0x11, 0x44, 0x11, 0x44, 0x11, 0x44, 0x11, 0x44], // 0xB0
    [0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA], // 0xB1
    [0x77, 0xDB, 0xEE, 0xDB, 0x77, 0xDB, 0xEE, 0xDB], // 0xB2
    [0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18], // 0xB3
    [0x18, 0x18, 0x18, 0x1F, 0x18, 0x18, 0x18, 0x18], // 0xB4
    [0x18, 0x18, 0x18, 0x1F, 0x18, 0x1F, 0x18, 0x18], // 0xB5
    [0x6C, 0x6C, 0x6C, 0x6F, 0x6C, 0x6C, 0x6C, 0x6C], // 0xB6
    [0x6C, 0x6C, 0x6C, 0x7F, 0x00, 0x00, 0x00, 0x00], // 0xB7
    [0x18, 0x18, 0x18, 0x1F, 0x18, 0x1F, 0x00, 0x00], // 0xB8
    [0x6C, 0x6C, 0x6C, 0x6F, 0x60, 0x6F, 0x6C, 0x6C], // 0xB9
    [0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C], // 0xBA
    [0x6C, 0x6C, 0x6C, 0x6F, 0x60, 0x7F, 0x00, 0x00], // 0xBB
    [0x00, 0x00, 0x00, 0x7F, 0x60, 0x6F, 0x6C, 0x6C], // 0xBC
    [0x00, 0x00, 0x00, 0x7F, 0x6C, 0x6C, 0x6C, 0x6C], // 0xBD
    [0x00, 0x00, 0x00, 0x1F, 0x18, 0x1F, 0x18, 0x18], // 0xBE
    [0x18, 0x18, 0x18, 0x1F, 0x00, 0x00, 0x00, 0x00], // 0xBF
    [0x00, 0x00, 0x00, 0xF8, 0x18, 0x18, 0x18, 0x18], // 0xC0
    [0x00, 0x00, 0x00, 0xFF, 0x18, 0x18, 0x18, 0x18], // 0xC1
    [0x18, 0x18, 0x18, 0xFF, 0x00, 0x00, 0x00, 0x00], // 0xC2
    [0x18, 0x18, 0x18, 0xF8, 0x18, 0x18, 0x18, 0x18], // 0xC3
    [0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00], // 0xC4
    [0x18, 0x18, 0x18, 0xFF, 0x18, 0x18, 0x18, 0x18], // 0xC5
    [0x18, 0x18, 0x18, 0xF8, 0x18, 0xF8, 0x18, 0x18], // 0xC6
    [0x6C, 0x6C, 0x6C, 0xEC, 0x6C, 0x6C, 0x6C, 0x6C], // 0xC7
    [0x00, 0x00, 0x00, 0xFC, 0x0C, 0xEC, 0x6C, 0x6C], // 0xC8
    [0x6C, 0x6C, 0x6C, 0xEC, 0x0C, 0xFC, 0x00, 0x00], // 0xC9
    [0x00, 0x00, 0x00, 0xFF, 0x00, 0xEF, 0x6C, 0x6C], // 0xCA
    [0x6C, 0x6C, 0x6C, 0xEF, 0x00, 0xFF, 0x00, 0x00], // 0xCB
    [0x6C, 0x6C, 0x6C, 0xEC, 0x0C, 0xEC, 0x6C, 0x6C], // 0xCC
    [0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00], // 0xCD
    [0x6C, 0x6C, 0x6C, 0xEF, 0x00, 0xEF, 0x6C, 0x6C], // 0xCE
    [0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x18, 0x18], // 0xCF
    [0x00, 0x00, 0x00, 0xFF, 0x6C, 0x6C, 0x6C, 0x6C], // 0xD0
    [0x18, 0x18, 0x18, 0xFF, 0x00, 0xFF, 0x00, 0x00], // 0xD1
    [0x6C, 0x6C, 0x6C, 0xFF, 0x00, 0x00, 0x00, 0x00], // 0xD2
    [0x00, 0x00, 0x00, 0xFC, 0x6C, 0x6C, 0x6C, 0x6C], // 0xD3
    [0x00, 0x00, 0x00, 0xF8, 0x18, 0xF8, 0x18, 0x18], // 0xD4
    [0x18, 0x18, 0x18, 0xF8, 0x18, 0xF8, 0x00, 0x00], // 0xD5
    [0x6C, 0x6C, 0x6C, 0xFC, 0x00, 0x00, 0x00, 0x00], // 0xD6
    [0x6C, 0x6C, 0x6C, 0xFF, 0x6C, 0x6C, 0x6C, 0x6C], // 0xD7
    [0x18, 0x18, 0x18, 0xFF, 0x18, 0xFF, 0x18, 0x18], // 0xD8
    [0x00, 0x00, 0x00, 0x1F, 0x18, 0x18, 0x18, 0x18], // 0xD9
    [0x18, 0x18, 0x18, 0xF8, 0x00, 0x00, 0x00, 0x00], // 0xDA
    [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], // 0xDB
    [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00], // 0xDC
    [0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F], // 0xDD
    [0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0], // 0xDE
    [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF], // 0xDF
    [0x00, 0x6E, 0x3B, 0x13, 0x3B, 0x6E, 0x00, 0x00], // 0xE0
    [0x03, 0x03, 0x1F, 0x33, 0x1F, 0x33, 0x1E, 0x00], // 0xE1
    [0x00, 0x03, 0x03, 0x03, 0x03, 0x33, 0x3F, 0x00], // 0xE2
    [0x00, 0x36, 0x36, 0x36, 0x36, 0x36, 0x7F, 0x00], // 0xE3
    [0x00, 0x3F, 0x33, 0x06, 0x0C, 0x06, 0x33, 0x3F], // 0xE4
    [0x00, 0x0E, 0x1B, 0x1B, 0x1B, 0x7E, 0x00, 0x00], // 0xE5
    [0x03, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x66, 0x00], // 0xE6
    [0x00, 0x18, 0x18, 0x18, 0x18, 0x3B, 0x6E, 0x00], // 0xE7
    [0x3F, 0x0C, 0x1E, 0x33, 0x33, 0x1E, 0x0C, 0x3F], // 0xE8
    [0x00, 0x1C, 0x36, 0x63, 0x7F, 0x63, 0x36, 0x1C], // 0xE9
    [0x00, 0x77, 0x36, 0x36, 0x63, 0x63, 0x36, 0x1C], // 0xEA
    [0x00, 0x1E, 0x33, 0x33, 0x3E, 0x18, 0x0C, 0x38], // 0xEB
    [0x00, 0x00, 0x7E, 0xDB, 0xDB, 0x7E, 0x00, 0x00], // 0xEC
    [0x03, 0x06, 0x7E, 0xDB, 0xDB, 0x7E, 0x30, 0x60], // 0xED
    [0x00, 0x1C, 0x06, 0x03, 0x1F, 0x03, 0x06, 0x1C], // 0xEE
    [0x00, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x1E], // 0xEF
    [0x00, 0x00, 0x3F, 0x00, 0x3F, 0x00, 0x3F, 0x00], // 0xF0
    [0x00, 0x3F, 0x00, 0x0C, 0x0C, 0x3F, 0x0C, 0x0C], // 0xF1
    [0x00, 0x3F, 0x00, 0x06, 0x0C, 0x18, 0x0C, 0x06], // 0xF2
    [0x00, 0x3F, 0x00, 0x18, 0x0C, 0x06, 0x0C, 0x18], // 0xF3
    [0x18, 0x18, 0x18, 0x18, 0x18, 0xD8, 0xD8, 0x70], // 0xF4
    [0x0E, 0x1B, 0x1B, 0x18, 0x18, 0x18, 0x18, 0x18], // 0xF5
    [0x00, 0x0C, 0x0C, 0x00, 0x3F, 0x00, 0x0C, 0x0C], // 0xF6
    [0x00, 0x00, 0x3B, 0x6E, 0x00, 0x3B, 0x6E, 0x00], // 0xF7
    [0x00, 0x00, 0x00, 0x00, 0x1C, 0x36, 0x36, 0x1C], // 0xF8
    [0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00], // 0xF9
    [0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00], // 0xFA
    [0x38, 0x3C, 0x36, 0x37, 0x30, 0x30, 0x30, 0xF0], // 0xFB
    [0x00, 0x00, 0x00, 0x36, 0x36, 0x36, 0x36, 0x1E], // 0xFC
    [0x00, 0x00, 0x00, 0x1E, 0x06, 0x0C, 0x18, 0x0E], // 0xFD
    [0x00, 0x00, 0x3C, 0x3C, 0x3C, 0x3C, 0x00, 0x00], // 0xFE
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 0xFF
];