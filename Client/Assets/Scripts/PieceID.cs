using System.Diagnostics;

public struct PieceID
{
    int ID;
    public PieceID(int v)
    {
        Debug.Assert(v != 0, "Can't create ID with value 0");
        ID = v;
    }
}
