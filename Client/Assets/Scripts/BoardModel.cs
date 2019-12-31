using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

using BoardPosition = BoardController.BoardPosition;
public class BoardModel
{
    public List<PieceModel> Pieces = new List<PieceModel>();

    public BoardModel()
    {
    }


    public void MovePiece(PieceModel pm, BoardPosition newPos)
    {
        pm.MoveTo(newPos);
    }


    public void FromNewGame(NewGameCreated ngc)
    {
        for(int i=0; i < ngc.game_state.map.grid.Count(); i++)
        {
            for(int j=0; j < ngc.game_state.map.grid.Count(); j++)
            {
                int id = ngc.game_state.map.grid[i][j];
                if (id != 0)
                {
                    Pieces.Add(new PieceModel(new PieceID(id),
                                              new BoardPosition(Tuple.Create(i, j)),
                                              ngc.game_state.objects[id]));
                }
            }
        }
    }
}
