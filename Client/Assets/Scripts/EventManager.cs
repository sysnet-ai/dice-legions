using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using UnityEngine;
using Utils;
public class EventManager : Singleton<EventManager>
{
    public class CallbackSubscription
    {
        public Func<Event, bool> Callback;
        public GameObject Requestor;

        public CallbackSubscription(Func<Event, bool> callback, GameObject req)
        {
            Callback = callback;
            Requestor = req;
        }
    }
    public Dictionary<Type, List<CallbackSubscription>> Subscriptions;

    void Awake()
    {
        Connection c = Connection.Instance;
        c.OnMessageEvent += HandleEventIncoming;
    }

    protected EventManager()
    {
        Subscriptions = new Dictionary<Type, List<CallbackSubscription>>();
    }

    public void SubscribeTo<T>(Func<Event, bool> cb, GameObject owner)
    {
        Type tt = typeof(T);
        List<CallbackSubscription> cbs = Subscriptions.GetOrDefault(tt, null);
        if (cbs == null)
        {
            cbs = new List<CallbackSubscription>();
            Subscriptions[tt] = cbs;
        }
        cbs.Add(new CallbackSubscription(cb, owner));
    }

    void HandleEventIncoming(string msg)
    {
        //TODO: Double deserializing seems dumb
        Event ev = (Event)JsonConvert.DeserializeObject(msg, typeof(Event));

        Type eventType = Type.GetType(ev.EventType);

        Debug.Log(eventType);

        if (eventType != null)
        {
            Event fullyParsedEvent = (Event)JsonConvert.DeserializeObject(msg, eventType);

            List<CallbackSubscription> subs = Subscriptions.GetOrDefault(eventType, null);
            if (subs != null)
            {
                foreach(var cbs in subs)
                {
                    if (cbs.Requestor != null)
                    {
                        cbs?.Callback.Invoke(fullyParsedEvent);
                    }
                    else
                    {
                        // Remove entry?
                    }
                }
            }
        }
    }
}
