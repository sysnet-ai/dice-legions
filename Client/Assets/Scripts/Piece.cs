using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using BoardPosition = BoardController.BoardPosition;
public class Piece : MonoBehaviour
{
    public PieceRenderer   Renderer;
    public Collider        Collider;


    public BoardController Board {get; private set;}
    public PieceModel Model {get; private set;}

    public void Initialize(PieceModel model, BoardController board) 
    {
        Board = board;
        Model = model;
    }


#region MonoBehaviour
    void Update()
    {
        gameObject.transform.localPosition = Model.Position.WorldPosition;
    }
#endregion

    public void Clicked()
    {
        Board.OnPieceClicked(this);
    }

    public void Select()
    {
        Renderer.Select();
        Collider.gameObject.SetActive(false);
    }

    public void Unselect()
    {
        Renderer.Unselect();
        Collider.gameObject.SetActive(true);
    }
}
