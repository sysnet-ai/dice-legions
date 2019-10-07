using System.Collections.Generic;
using UnityEngine;

public class PieceRenderer : MonoBehaviour
{
    public Material SelectedMaterial;
    public List<Renderer> Renderers;

    private Material DefaultMaterial;

    void Start()
    {
        DefaultMaterial = Renderers[0].material;
    }

    public void Select()
    {
        foreach(Renderer r in Renderers)
        {
            r.material = SelectedMaterial;
        }
    }

    public void Unselect()
    {
        foreach(Renderer r in Renderers)
        {
            r.material = DefaultMaterial;
        }
    }
} 
