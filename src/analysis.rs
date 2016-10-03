#![allow(dead_code)]
use geometry::Square;
use color::Color;
use rank::Rank;
use piece::Piece;
use castle::Castle;

enum MoveAnnotations {
    None,
    Promotion,
    Capture,
    EnPassant,
    DoublePush,
}

enum Warnings {
    None,
    MissingPromotionHint,
    SparePromotion,
}

enum Errors {
    None,

    MoveToCheck,
    FromEmptyCell,
    ToOccupiedCell,
    WrongSideToMove,

    CastleFromCheck,
    CastleThroughCheck,
    HasNoCastling,

    HasNoEnPassant,

    DoesNotMoveThisWay,
    DoesNotCaptureThisWay,
    OnlyCapturesThisWay,
    JumpsOverPieces,
}

enum BoardValidationWarnings {
    TooManyPawns(Color),
    TooManyPieces(Color),
    PawnsOnBackrank(Color),
}

enum BoardValidationErrors {
    OppositeCheck, // active side can take the king this move

    TooManyKings(Color),
    MissingKing(Color),

    WrongEnPassantTarget(Rank), // 5th rank square is e.g. rook
    MissingEnPassantTarget(Square), // 5th rank square  is empty
    BlockedEnPassantTarget(Square, Piece), // 6th rank square is occupied

    NoKingToCastle(Castle),
    NoRookToCastle(Castle),
}
