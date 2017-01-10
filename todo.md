# TODO


1. list of all available moves in position
    2. -> list of moves
        - expand promotions (for engines, can be optional?)

1. from what cells can a move be made with the given piece type to this cell
    (for SAN disambiguation)
    - valid pawn captures to the cell
    - valid pawn pushes to the cell
    - other

3. What moves a particular piece has (mask)
    3. -> mask
        - exactly what UI needs
        - there's no redundant bit: from square
        - represents castling as a long king move (still what UI needs)
    4. -> list of moves
        - some info is not lost:
            - that a move is promotion
            - is take \ en-passant
            - is double push
            - check \ mate
            - is castling
    5. -> list of moves from square possibilities
    6. -> expand promotions?
        - engines would use list of all moves anyway?
4. Validate a move
    -  -> bool: for SAN
    -  -> error: for UI