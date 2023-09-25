Feature: Block peers

  Scenario: A block peer attempt to connect
    Given the following peers are started
      | Name | Port |
      | P0   | 9100 |
      | P1   | 9101 |
      | P2   | 9102 |
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
    And the peer "P1" sends "I am a peer" to all
    Then the peer "P0" receives
      | Event   | Content     | From |
      | MESSAGE | I am a peer | P1   |
    Then the peer "P2" does not receives
      | Event   | Content     | From |
      | MESSAGE | I am a peer | P1   |
    When the peer "P1" unblocks the peer "P2"
    And the peer "P1" sends "Hello" to all
    Then the peer "P2" receives
      | Event   | Content | From |
      | MESSAGE | Hello   | P1   |
