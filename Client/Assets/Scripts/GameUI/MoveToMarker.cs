using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using BoardPosition = BoardController.BoardPosition;

public class MoveToMarker : MonoBehaviour
{
    public BoardPosition Position {get; private set; }
    public BoardController Controller {get; private set; }


    public void Initialize(BoardPosition bPos, BoardController controller)
    {
        Position = bPos;
        Controller = controller;
        gameObject.transform.localPosition = Position.WorldPosition;
    }

    public void  Clicked()
    {
        Controller?.OnMoveToMarkerClicked(this);
    }
}
