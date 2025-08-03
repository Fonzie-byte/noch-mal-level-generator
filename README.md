# Noch Mal! level generator
Generates random, valid “Noch Mal!” levels.

The game is also known as “Keer op Keer!” in Nederlands/Dutch, and "Encore!" in English and français/French.

“Valid” appears to mean:
- 15 columns, lettered 'A' through 'O'
- Each column contains:
  - 7 cells
  - Containing 1 or more of *each* colour
  - *Exactly* 1 of the cells has a star
- Each “row” also contains each colour and at least 1 star
- For each colour, *exactly* 21 cells exist, in groups of 1, 2, 3, 4, 5 and 6 cells.
  - 1 of each group exists
  - “Groups” can be identified as a series made up from horizontally and/or vertically linked cells of the same colour
