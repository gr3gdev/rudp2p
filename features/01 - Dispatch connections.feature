Feature: Dispatch connections

  Scenario: Reception of connection and disconnection events
    Given the following peers are started
      | Name | Port |
      | P0   | 9000 |
      | P1   | 9001 |
      | P2   | 9002 |
      | P3   | 9003 |
    When the peer "P1" connects to "P0"
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P1   |
    And the peer "P1" receives
      | Event     | From |
      | CONNECTED | P0   |
    When the peer "P2" connects to "P0"
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P2   |
    And the peer "P1" receives
      | Event     | From |
      | CONNECTED | P2   |
    And the peer "P2" receives
      | Event     | From |
      | CONNECTED | P0   |
      | CONNECTED | P1   |
    When the peer "P3" connects to "P0"
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P3   |
    And the peer "P1" receives
      | Event     | From |
      | CONNECTED | P3   |
    And the peer "P2" receives
      | Event     | From |
      | CONNECTED | P3   |
    And the peer "P3" receives
      | Event     | From |
      | CONNECTED | P0   |
      | CONNECTED | P1   |
      | CONNECTED | P2   |
    When the peer "P2" disconnects
    Then the peer "P0" receives
      | Event        | From |
      | DISCONNECTED | P2   |
    And the peer "P1" receives
      | Event        | From |
      | DISCONNECTED | P2   |
    And the peer "P3" receives
      | Event        | From |
      | DISCONNECTED | P2   |
    And the peer "P2" receives
      | Event        | From |
      | DISCONNECTED | P0   |
      | DISCONNECTED | P1   |
      | DISCONNECTED | P3   |
