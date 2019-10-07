using UnityEngine;

#if UNITY_EDITOR
using UnityEditor;
#endif

[System.Serializable]
public class UnityLayerSelection
{
    [SerializeField]
    private int m_LayerIndex = 0;
    public int LayerIndex
    {
        get { return m_LayerIndex; }
    }

    public int Mask
    {
        get { return 1 << m_LayerIndex; }
    }
}


#if UNITY_EDITOR
 [CustomPropertyDrawer(typeof(UnityLayerSelection))]
 public class UnityLayerSelectionPropertyDrawer : PropertyDrawer 
 {
     public override void OnGUI(Rect _position, SerializedProperty _property, GUIContent _label)
     {
         EditorGUI.BeginProperty(_position, GUIContent.none, _property);
         SerializedProperty layerIndex = _property.FindPropertyRelative("m_LayerIndex");
         _position = EditorGUI.PrefixLabel(_position, GUIUtility.GetControlID(FocusType.Passive), _label);
         if (layerIndex != null)
         {
             layerIndex.intValue = EditorGUI.LayerField(_position, layerIndex.intValue);
         }
         EditorGUI.EndProperty( );
     }
 }
#endif
