using System.Collections.Generic;

public static class CollectionExtensions
{
    public static V GetOrDefault<K, V>(this IDictionary<K,V> dict, K key, V defaultValue)
    {
        if (dict.ContainsKey(key))
        {
            return dict[key];
        }

        return defaultValue;
    }
}
