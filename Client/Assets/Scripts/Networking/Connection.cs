using UnityEngine;
using WebSocketSharp;
using System;
using System.Collections;
using System.Collections.Generic;
using Utils;


public class Connection : Singleton<Connection>
{
    public delegate void MessageReceived(string message);
    public event MessageReceived OnMessageEvent;
    private WebSocket _ws;
    
    List<string> MessagesToProcess = new List<string>();

    public WebSocket ws {
        get
        {
            return _ws;
        }
        set
        {
            _ws = value;
        }
    }
    
    public string Address { get; private set; }

    protected Connection()
    {
        // the url to sisnett's Amazon EC2 linux box
        //TODO: Make this configurable
        //Address = "ws://ec2-54-70-6-182.us-west-2.compute.amazonaws.com:8080";
        Address = "ws://localhost:8080";
        ws = new WebSocket(Address);
        Debug.Log(ws.Protocol);
        Debug.Log("Initilizing Connection to: " + Address);
        ws.EmitOnPing = true;
        ws.OnMessage += (sender, e) => OnMessage(e.Data);
        ws.Connect();
    }


    void OnMessage(string message)
    {
        Debug.Log(message);
        MessagesToProcess.Add(message);
    }
    
    void Update()
    {
        if (MessagesToProcess.Count > 0)
        {
            string message = MessagesToProcess[0];
            MessagesToProcess.RemoveAt(0);

            Debug.Log("Eventing");
            OnMessageEvent?.Invoke(message);
        }
    }

    public void Send(string content)
    {
        if (ws != null)
        {
            ws.Send(content);
        }
    }
    public void CloseConnection()
    {
        ws.Close();
        Debug.Log("CLOSING CONNECTION!");
    }

    public override void OnDestroy()
    {
        CloseConnection();
        base.OnDestroy();
    }
}
