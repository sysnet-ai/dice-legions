using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class BoardController : MonoBehaviour
{
    public struct BoardPosition
    {
        public Vector3 WorldPosition {get; private set;}

        public float WorldX {get { return WorldPosition.x; } }
        public float WorldZ {get { return WorldPosition.z; } }

        public Tuple<int, int> GridPosition { get { return new Tuple<int, int>((int)WorldX, (int)WorldZ); } }

        public BoardPosition(Vector3 pos)
        {
            WorldPosition = pos;
        }

        public BoardPosition(Tuple<int, int> gridPos)
        {
            WorldPosition = new Vector3(gridPos.Item1, 0.0f, gridPos.Item2);
        }

        public BoardPosition Offset(Tuple<int, int> offset)
        {
            var newPos = WorldPosition + new Vector3(offset.Item1, 0.0f, offset.Item2);
            return new BoardPosition(newPos);
        }
    }


////////////////////////////////////////////////////////////////////////////////


    public MoveToMarker MarkerPrototype;
    public Piece        PiecePrototype;

    public BoardModel   Model {get; private set;}

    private List<MoveToMarker> InstantiatedMarkers = new List<MoveToMarker>();
    private Piece ClickedPiece = null;


#region MonoBehaviour
    void Awake()
    {
        Debug.Log("Awoken!");
    }

    void Start()
    {
        Debug.Log("Started!");

        EventManager em = EventManager.Instance;
        em.SubscribeTo<NewGameCreated>(NewGameCreated, this.gameObject);

        Connection WSConnection = Connection.Instance;

        // TODO: Not sure this goes here, change to object from string
        WSConnection.Send("{ \"ActionType\": \"NewGame\" }" );
        
        Model = new BoardModel();
    }
#endregion

#region Handlers
    public bool NewGameCreated(Event ev)
    {
        NewGameCreated ngc = ev as NewGameCreated;
        System.Diagnostics.Debug.Assert(ngc != null, "Passed wrong event type");

        Model.FromNewGame(ngc);

        foreach(PieceModel pm in Model.Pieces)
        {
            Piece p = Instantiate(PiecePrototype);
            p.Initialize(pm, this);
        }
        return true;
    }


    public void OnPieceClicked(Piece piece)
    {
        ConfigurableLogger.Debug(piece.Model.Position.GridPosition);

        ClearMarkers();

        // Find all valid moves for that piece
        // TODO: Cheaty mcCheatyface
        var deltas = new List<Tuple<int, int>>() {
            Tuple.Create(-1, -1),
            Tuple.Create( 1, -1),
            Tuple.Create( 1,  1),
            Tuple.Create(-1,  1),
        };

        ClickedPiece = piece;
        ClickedPiece.Select();

        var moves = new List<BoardPosition>();
        foreach(var d in deltas)
        {
            BoardPosition bp = piece.Model.Position.Offset(d);
            moves.Add(bp);
            ConfigurableLogger.Debug(bp.GridPosition);
        }

        // Spawn MoveToMarkers
        foreach(var m in moves)
        {
            MoveToMarker mtm = Instantiate(MarkerPrototype);
            mtm.Initialize(m, this);
            InstantiatedMarkers.Add(mtm);
        }
    }

    public void OnMoveToMarkerClicked(MoveToMarker clickedMarker)
    {
        if (ClickedPiece == null)
        {
            ConfigurableLogger.LogError("A Marker Clicked without a Selected Piece");
        }

        ConfigurableLogger.Debug(clickedMarker.Position.GridPosition);

        //  Move the Piece on the Model
        Model.MovePiece(ClickedPiece.Model, clickedMarker.Position);

        //  Tell the Piece to Move
        ClearMarkers();
    }

    public void DismissSelection()
    {
        ClearMarkers();
    }
#endregion

    private void ClearMarkers()
    {
        foreach(var m in InstantiatedMarkers)
        {
            Destroy(m.gameObject);
        }

        InstantiatedMarkers = new List<MoveToMarker>();
        ClickedPiece?.Unselect();
        ClickedPiece = null;
    }
}
