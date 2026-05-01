ChronoBinTern
Експериментален хибриден изчислителен модел: двоична и балансирана троична логика чрез времево кодиране

Experimental Hybrid Computational Model: Binary and Balanced Ternary Logic via Temporal Encoding

Концепция
Проектът изследва метод за времево кодиране, при който 4-битова двоична дума (nibble) носи едновременно и балансирана троична стойност. Знакът, модулът и троичният вектор се вграждат в продължителността и посоката на тактовите импулси, без допълнителен бит.

Concept
The project explores a temporal encoding method where a 4-bit binary word (nibble) simultaneously carries a balanced ternary value. The sign, modulus, and ternary vector are embedded in the duration and direction of clock pulses, without an extra bit.

Разширен обхват:
16-те стандартни двоични комбинации (0–15) са разширени с отрицателни стойности от -1 до -11, като нулата е запазена. Така се получават 27 уникални символа, които запазват пълната 4-битова адресация.

Extended Range:
The 16 standard binary combinations (0–15) are expanded with negative values from -1 to -11, while preserving zero. This yields 27 unique symbols that retain full 4-bit addressability.

Съответствие с балансирана троична система:
27-те символа се напасват без загуба със стойностите на балансирана троична система в диапазона -13 до +13 (3 трита, 27 комбинации).

Mapping to Balanced Ternary:
The 27 symbols map losslessly to the values of a balanced ternary system in the range -13 to +13 (3 trits, 27 combinations).

Комбинаторно пространство:
Всеки от 27-те символа може да бъде съчетан с всяка от 27-те троични стойности, което поражда 27 × 27 = 729 варианта за кодиране. Всеки вариант представлява отделна конфигурация на времевия модел (тактове работа, паузи, посока).

Combinatorial Space:
Each of the 27 symbols can be paired with any of the 27 ternary values, resulting in 27 × 27 = 729 variants of encoding. Each variant is a separate configuration of the temporal pattern (work cycles, pauses, direction).

Това позволява едновременно изпълнение на двоични и троични инструкции върху обща виртуална машина, без превключване на режима.

This enables simultaneous execution of binary and ternary instructions on a shared virtual machine, without mode switching.

Статус
Проектът е в ранна изследователска фаза. Цели:

Документиране на вариантите на кодиране.

Анализ на информационната плътност и скоростта.

Софтуерен емулатор (виртуална машина).

Изследване на приложения в невронни мрежи.

Status
Early-stage research project. Goals:

Document encoding variants.

Analyze information density and transfer speed.

Build a software emulator (virtual machine).

Explore applications in neural networks.

Лиценз
GNU Affero General Public License v3.0 (AGPL-3.0)


Dec | Nibble (Bin)  | Bal3 (t)  | Работа (|t|)  | Пауза | Общо | Hex
-----|---------------|-----------|---------------|-------|------|-----
 -11 |     1011      |  +1 +1 +1 |    13 (F)     |  -24  |  37  | 0x0B
 -10 |     1010      |  +1 +1  0 |    12 (F)     |  -22  |  34  | 0x0A
  -9 |     1001      |  -1 -1 -1 |  - 13 (R)     |   4   |  17  | 0x09
  -8 |     1000      |  -1 -1  0 |  - 12 (R)     |   4   |  16  | 0x08
  -7 |     0111      |  -1 -1 +1 |  - 11 (R)     |   4   |  15  | 0x07
  -6 |     0110      |  -1  0 -1 |  - 10 (R)     |   4   |  14  | 0x06
  -5 |     0101      |  -1  0  0 |  -  9 (R)     |   4   |  13  | 0x05
  -4 |     0100      |  -1  0 +1 |  -  8 (R)     |   4   |  12  | 0x04
  -3 |     0011      |  -1 +1 -1 |  -  7 (R)     |   4   |  11  | 0x03
  -2 |     0010      |  -1 +1  0 |  -  6 (R)     |   4   |  10  | 0x02
  -1 |     0001      |  -1 +1 +1 |  -  5 (R)     |   4   |   9  | 0x01
   0 |     0000      |   0 -1 -1 |  -  4 (R)     |   4   |   8  | 0x00
   1 |     0001      |   0 -1  0 |  -  3 (R)     |   4   |   7  | 0x01
   2 |     0010      |   0 -1 +1 |  -  2 (R)     |   4   |   6  | 0x02
   3 |     0011      |   0  0 -1 |  -  1 (R)     |   4   |   5  | 0x03
   4 |     0100      |   0  0  0 |     0 (P)     |   4   |   5  | 0x04
   5 |     0101      |   0  0 +1 |     1 (F)     |   4   |   5  | 0x05
   6 |     0110      |   0 +1 -1 |     2 (F)     |   4   |   6  | 0x06
   7 |     0111      |   0 +1  0 |     3 (F)     |   4   |   7  | 0x07
   8 |     1000      |   0 +1 +1 |     4 (F)     |   4   |   8  | 0x08
   9 |     1001      |  +1 -1 -1 |     5 (F)     |   4   |   9  | 0x09
  10 |     1010      |  +1 -1  0 |     6 (F)     |   4   |  10  | 0x0A
  11 |     1011      |  +1 -1 +1 |     7 (F)     |   4   |  11  | 0x0B
  12 |     1100      |  +1  0 -1 |     8 (F)     |   4   |  12  | 0x0C
  13 |     1101      |  +1  0  0 |     9 (F)     |   4   |  13  | 0x0D
  14 |     1110      |  +1  0 +1 |    10 (F)     |   4   |  14  | 0x0E
  15 |     1111      |  +1 +1 -1 |    11 (F)     |   4   |  15  | 0x0F
  
  
  
  27*27=729   727/729
