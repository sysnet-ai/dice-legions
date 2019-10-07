using System;
using UnityEngine;

public class ConfigurableLogger
{
    public enum LogLevel { Info, Debug };

    public static LogLevel MaxLevel = LogLevel.Info;

    static void LogAtLevel(LogLevel lvl, string v)
    {
        if (lvl <= MaxLevel)
        {
#if UNITY_EDITOR
            UnityEngine.Debug.Log(v);
#else
            Console.WriteLine(v);
#endif
        }
    } 

    public static void LogError(object v)
    {
#if UNITY_EDITOR
            UnityEngine.Debug.LogError(v);
#else
            Console.WriteLine($"ERROR: {v});
#endif
    }

    public static void Log(object v)
    {
        LogAtLevel(LogLevel.Info, v.ToString());
    }

    public static void Debug(object v)
    {
        LogAtLevel(LogLevel.Debug, v.ToString());
    }
}
