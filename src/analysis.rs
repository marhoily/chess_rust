#![allow(dead_code)]

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

