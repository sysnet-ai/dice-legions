using System.Collections.Generic;

using BoardPosition = BoardController.BoardPosition;
public class PieceModel
{
    public BoardPosition Position {get; private set; }
    public PieceID       ID { get; private set; }
    public PlayerID      Owner { get; private set; }

    public Dictionary<StatID, int> Stats { get; private set; }

    public PieceModel(PieceID id, BoardPosition pos, PieceStats stats)
    {
        Position = pos;
        ID = id;

        Stats = new Dictionary<StatID, int>();
        Owner = stats.owner;
        foreach(var statsKvp in stats.stats)
        {
            Stats[statsKvp.Key] = statsKvp.Value;
        }
    }

    public void MoveTo(BoardPosition pos)
    {
        Position = pos;
    }
}
