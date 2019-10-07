using UnityEngine;
using UnityEngine.Events;

public class ClickHandler : MonoBehaviour
{
    public UnityEvent OnClicked;

    public void Clicked()
    {
        OnClicked.Invoke();
    }
}
