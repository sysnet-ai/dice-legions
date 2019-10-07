using UnityEngine;
using UnityEngine.Events;

public class InputController : MonoBehaviour
{
    public UnityLayerSelection[] LayerPriorities;

    void Update()
    {
        if (Input.GetMouseButtonDown(0))
        { 
            for(int l = 0; l < LayerPriorities.Length; l++)
            {
                Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
                RaycastHit[] hits = Physics.RaycastAll(ray, Mathf.Infinity, LayerPriorities[l].Mask);

                for(int i = 0; i < hits.Length; i++)
                {
                    hits[i].collider.gameObject.GetComponent<ClickHandler>()?.Clicked();
                    ConfigurableLogger.Debug($"Hit {hits[i].collider.gameObject.name}");
                }

                if (hits.Length > 0)
                {
                    break;
                }
            }
        }
    }
}
