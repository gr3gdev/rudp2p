# Documentation of the library

## Dispatcher
Manage connections.


### New connection *Peer1*
```mermaid
flowchart LR
  classDef peer stroke:#0f0;
  
  P1[fa:fa-laptop Peer1]:::peer --Connecting--> B((fa:fa-server Dispatcher))
  
  B --Peer1 Connected--> P1
```

### New connection *Peer2*
```mermaid
flowchart LR
  classDef peer stroke:#0f0;
  
  P2[fa:fa-laptop Peer2]:::peer --Connecting--> B((fa:fa-server Dispatcher))
  
  B --Peer2 Connected--> P1[fa:fa-laptop Peer1]:::peer
  B --Peer2 Connected--> P2
  B --Peer1 Connected--> P2
```

### New connection *Peer3*
```mermaid
flowchart LR
  classDef peer stroke:#0f0;
  
  P3[fa:fa-laptop Peer3]:::peer --Connecting--> B((fa:fa-server Dispatcher))
  
  B --Peer3 Connected--> P1[fa:fa-laptop Peer1]:::peer
  B --Peer3 Connected--> P2[fa:fa-laptop Peer2]:::peer
  B --Peer3 Connected--> P3
  B --Peer1 Connected--> P3
  B --Peer2 Connected--> P3
```

### *Peer3* disconnecting
```mermaid
flowchart LR
  classDef peer stroke:#0f0;
  
  P3[fa:fa-laptop Peer3]:::peer --Disconnecting--> B((fa:fa-server Dispatcher))
  
  B --Peer3 Disconnected--> P1[fa:fa-laptop Peer1]:::peer
  B --Peer3 Disconnected--> P2[fa:fa-laptop Peer2]:::peer
```

## Exchange

### *Peer1* Send message to all
```mermaid
flowchart LR
  classDef peer stroke:#0f0;
  
  P1[fa:fa-laptop Peer1]:::peer --Message--> P2[fa:fa-laptop Peer2]:::peer
  P1[fa:fa-laptop Peer1]:::peer --Message--> P3[fa:fa-laptop Peer3]:::peer
```
