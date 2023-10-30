Feature: Block peers

  Scenario: A block peer does not receive any messages until he has unblock
    Given the following peers are started
      | Name | Port | Database |
      | P0   | 9100 | InMemory |
      | P1   | 9101 | Sqlite   |
      | P2   | 9102 | InMemory |
    When the peer "P1" connects to "P0"
    Then the peer "P1" receives
      | Event     | From |
      | CONNECTED | P0   |
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P1   |
    When the peer "P2" connects to "P0"
    Then the peer "P1" receives
      | Event     | From |
      | CONNECTED | P2   |
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P2   |
    Then the peer "P2" receives
      | Event     | From |
      | CONNECTED | P0   |
      | CONNECTED | P1   |
    When the peer "P1" blocks the peer "P2"
    Then the peer "P2" receives
      | Event        | From |
      | DISCONNECTED | P1   |
    When the peer "P1" sends "I am a peer" to "all"
    Then the peer "P0" receives
      | Event   | Content     | From |
      | MESSAGE | I am a peer | P1   |
    Then the peer "P2" does not receives
      | Event   | Content     | From |
      | MESSAGE | I am a peer | P1   |
    When the peer "P1" unblocks the peer "P2"
    Then the peer "P2" receives
      | Event     | From |
      | CONNECTED | P1   |
    When the peer "P1" sends "Hello" to "all"
    Then the peer "P2" receives
      | Event   | Content | From |
      | MESSAGE | Hello   | P1   |

  Scenario: A block peer can not send any messages until he has unblock
    Given the following peers are started
      | Name | Port | Database                                 |
      | P0   | 9200 | Sqlite                                   |
      | P1   | 9201 | InMemory                                 |
      | P2   | 9202 | mysql://cucumber:test@localhost:33062/P2 |
    When the peer "P1" connects to "P0"
    Then the peer "P1" receives
      | Event     | From |
      | CONNECTED | P0   |
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P1   |
    When the peer "P2" connects to "P0"
    Then the peer "P1" receives
      | Event     | From |
      | CONNECTED | P2   |
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P2   |
    Then the peer "P2" receives
      | Event     | From |
      | CONNECTED | P0   |
      | CONNECTED | P1   |
    When the peer "P2" blocks the peer "P1"
    Then the peer "P1" receives
      | Event        | From |
      | DISCONNECTED | P2   |
    When the peer "P1" sends "I am a peer" to "all"
    Then the peer "P0" receives
      | Event   | Content     | From |
      | MESSAGE | I am a peer | P1   |
    Then the peer "P2" does not receives
      | Event   | Content     | From |
      | MESSAGE | I am a peer | P1   |
    When the peer "P2" unblocks the peer "P1"
    Then the peer "P1" receives
      | Event     | From |
      | CONNECTED | P2   |
    When the peer "P1" sends "Hello" to "all"
    Then the peer "P2" receives
      | Event   | Content | From |
      | MESSAGE | Hello   | P1   |
