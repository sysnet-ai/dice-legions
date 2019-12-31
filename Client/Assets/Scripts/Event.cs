using System;
using System.Collections.Generic;

[Serializable]
public class Event
{
    public string EventType;
}

[Serializable]
public enum StatID
{
    Health,
    Speed
}

[Serializable]
public enum PlayerID
{
    Player1,
    Player2
}

[Serializable]
public class PieceStats
{
    public PlayerID owner;
    public Dictionary<StatID, int> stats;
}

[Serializable]
public class GameState
{
    public Map map;
    public Dictionary<int, PieceStats> objects;
}

[Serializable]
public class Map
{
    public List<List<int>> grid;
}

[Serializable]
public class NewGameCreated : Event
{
    public GameState game_state;
}
