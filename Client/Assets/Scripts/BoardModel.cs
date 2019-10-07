using System;
using System.Collections.Generic;

using BoardPosition = BoardController.BoardPosition;
public class BoardModel
{
    public List<PieceModel> Pieces; 

    public BoardModel()
    {
        // TODO: Cheaty mcCheatyFace
        Pieces = new List<PieceModel>() 
        {
            new PieceModel(new BoardPosition(Tuple.Create(4, 0))),
            new PieceModel(new BoardPosition(Tuple.Create(4, 4))),
            new PieceModel(new BoardPosition(Tuple.Create(9, 9)))
        };
    }


    public void MovePiece(PieceModel pm, BoardPosition newPos)
    {
        pm.MoveTo(newPos);
    }
}
