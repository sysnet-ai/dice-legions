using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

using BoardPosition = BoardController.BoardPosition;

[RequireComponent(typeof(MeshFilter), typeof(MeshRenderer))]
public class MeshGenerator : MonoBehaviour
{
    //
    Mesh GeneratedMesh;
    Vector3[] Vertices;
    

    void Awake()
    {
        GeneratedMesh = GetComponent<MeshFilter>().mesh;

        //TODO: Cheaty McCheatyFace
        GenerateMesh(new BoardPosition[] { new BoardPosition(new Tuple<int, int>(0, 0)),
                                           new BoardPosition(new Tuple<int, int>(0, 1)),
                                           new BoardPosition(new Tuple<int, int>(1, 1)),
                                           new BoardPosition(new Tuple<int, int>(1, 2)),
                                           new BoardPosition(new Tuple<int, int>(1, 3)) });
    }

    private void OnDrawGizmos()
    {
        if (Vertices == null)
        {
            return;
        }

		Gizmos.color = Color.black;
		for (int i = 0; i < Vertices.Length; i++)
        {
			Gizmos.DrawSphere(Vertices[i], 0.1f);
		}
    }

    void GenerateMesh(BoardPosition[] poss)
    { 
        Vertices = new Vector3[poss.Length * 4];
        int[] triangles = new int[poss.Length * 6];
        Vector2[] uvs = new Vector2[Vertices.Length];

        //TODO... Static? Class?
        Vector2[] upUvs = new Vector2[] {
            new Vector2(0.0f, 0.0f),
            new Vector2(0.5f, 0.0f),
            new Vector2(0.5f, 0.5f),
            new Vector2(0.0f, 0.5f),
        }; 

        Vector2[] rightUvs = new Vector2[] {
            new Vector2(0.0f, 0.5f),
            new Vector2(0.5f, 0.5f),
            new Vector2(0.5f, 1.0f),
            new Vector2(0.0f, 1.0f),
        };
        
        for(int pi = 0, vi = 0, ti = 0; pi < poss.Length; pi++, vi+=4, ti+=6)
        {
            BoardPosition pos = poss[pi];

            BoardPosition next = pos;
            if (pi < poss.Length - 1)
            {
                next = poss[pi+1];
            }
            bool goesUp    = next.WorldZ > pos.WorldZ;
            bool goesDown  = next.WorldZ < pos.WorldZ;
            bool goesLeft  = next.WorldX < pos.WorldX;
            bool goesRight = next.WorldX > pos.WorldX;

            ConfigurableLogger.MaxLevel = ConfigurableLogger.LogLevel.Debug;
            ConfigurableLogger.Debug($"Up {goesUp} Down {goesDown} Left {goesLeft} Right {goesRight}");
            ConfigurableLogger.MaxLevel = ConfigurableLogger.LogLevel.Info;

            //TODO... Where does these come from?
            float offset_x = 1.0f;
            float offset_z = 1.0f;

            // Create all vertices     
            Vector3 v1 = new Vector3(pos.WorldX, 0.0f, pos.WorldZ);
            Vector3 v2 = new Vector3(pos.WorldX + offset_x, 0.0f, pos.WorldZ);
            Vector3 v3 = new Vector3(pos.WorldX + offset_x, 0.0f, pos.WorldZ + offset_z);
            Vector3 v4 = new Vector3(pos.WorldX, 0.0f, pos.WorldZ + offset_z);

            Vertices[vi]   = v1;
            Vertices[vi+1] = v2;
            Vertices[vi+2] = v3;
            Vertices[vi+3] = v4;

            // Add UV's
            Vector2[] useUvs;
            if (goesUp)
            {
                useUvs = upUvs;
            }
            else
            {
                useUvs = rightUvs;
            }
            uvs[vi]   = useUvs[0]; 
            uvs[vi+1] = useUvs[1]; 
            uvs[vi+2] = useUvs[2]; 
            uvs[vi+3] = useUvs[3]; 

            // Create triangles
            // 1,4,2
            // 3,2,4
            triangles[ti]   = vi;
            triangles[ti+1] = triangles[ti+5] = vi+3;
            triangles[ti+2] = triangles[ti+4] = vi+1;
            triangles[ti+3] = vi+2;
        }

        GeneratedMesh.vertices = Vertices;
        GeneratedMesh.triangles = triangles;
        GeneratedMesh.uv = uvs;
    }
}
