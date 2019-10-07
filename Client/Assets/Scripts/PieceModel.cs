using BoardPosition = BoardController.BoardPosition;
public class PieceModel
{
    public BoardPosition Position {get; private set; }
    public PieceID       ID { get; private set; }

    public PieceModel(BoardPosition pos)
    {
        Position = pos;
    }

    public void MoveTo(BoardPosition pos)
    {
        Position = pos;
    }
}
