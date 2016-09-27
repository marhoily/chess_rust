// Contains functions to work with the FEN
// representation of chess positions
// http://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation

struct File(i8);
struct Rank(i8);
struct Index64(i8);

enum Color {
    Black = 0b01_000000,
    White = 0b10_000000,
}
enum PieceType {
    None = 0b00_000000,
    Pawn = 0b00_000001,
    Knight = 0b00_000010,
    Bishop = 0b00_000100,
    Rook = 0b00_001000,
    Queen = 0b00_010000,
    King = 0b00_100000,
}
enum Piece {
    EmptyCell = 0,
    WhitePawn = 0b10_000001,
    WhiteKnight = 0b10_000010,
    WhiteBishop = 0b10_000100,
    WhiteRook = 0b10_001000,
    WhiteQueen = 0b10_010000,
    WhiteKing = 0b10_100000,
    BlackPawn = 0b01_000001,
    BlackKnight = 0b01_000010,
    BlackBishop = 0b01_000100,
    BlackRook = 0b01_001000,
    BlackQueen = 0b01_010000,
    BlackKing = 0b01_100000,
}

enum Castlings {
    None = 0b0000,
    Q = 0b0101,
    K = 0b1010,
    W = 0b0011,
    B = 0b1100,
    WQ = 0b0001,
    WK = 0b0010,
    BQ = 0b0100,
    BK = 0b1000,
    All = 0b1111,
}

enum GameStates {
    None = 0b0000000,
    Check = 0b00000_01,
    Mate = 0b00000_10,
    Draw = 0b1_1111_11,
    Repetition = 0b1_0001_00,
    FiftyMoveRule = 0b1_0010_00,
    Stalemate = 0b1_0100_00,
    InsufficientMaterial = 0b1_1000_00,
}

struct Move {
    From: Index64,
    To: Index64,
    PromoteTo: PieceType,
}

enum MoveAnnotations {
    None = 0b0000,
    Promotion = 0b0001,
    Capture = 0b0010,
    EnPassant = 0b0100,
    DoublePush = 0b1000,
}

enum Warnings {
    None = 0b00,
    MissingPromotionHint = 0b01,
    SparePromotion = 0b10,
}

enum Errors {
    None = 0,

    MoveToCheck = 0b000000000001,
    FromEmptyCell = 0b000000000010,
    ToOccupiedCell = 0b000000010000,
    WrongSideToMove = 0b000000000100,

    CastleFromCheck = 0b100000000000,
    CastleThroughCheck = 0b001000000000,
    HasNoCastling = 0b000000001000,

    HasNoEnPassant = 0b000000100000,

    DoesNotMoveThisWay = 0b010000000000,
    DoesNotCaptureThisWay = 0b000100000000,
    OnlyCapturesThisWay = 0b000010000000,
    JumpsOverPieces = 0b000001000000,
}

struct PositionCore {
    // Note that index 0 corresponds to a8, and NOT a1!
    // Indexes read left to right, top to bottom!
    Squares: [Piece; 64],
    // The color of the side that makes the next move
    ActiveColor: Color,
    AvailableCastlings: Castlings,
    EnPassant: Option<File>,
}
