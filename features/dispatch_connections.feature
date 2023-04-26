Feature: Dispatch connections

  Scenario: Connection and disconnection
    Given the following peers are started
      | Name       | Port |
      | P0 | 9000 |
    When the following peers connect to "P0"
      | Name | Port |
      | P1   | 9001 |
    Then the peer "P0" receives
      | Event     | From |
      | Connected | P1   |
    Then the peer "P1" receives
      | Event     | From |
      | Connected | P0   |
    When the following peers connect to "P0"
      | Name | Port |
      | P2   | 9002 |
    Then the peer "P0" receives
      | Event     | From |
      | Connected | P2   |
    And the peer "P2" receives
      | Event     | From |
      | Connected | P0   |
      | Connected | P1   |
    Then the peer "P1" receives
      | Event     | From |
      | Connected | P2   |
    When the following peers connect to "P0"
      | Name | Port |
      | P3   | 9003 |
    Then the peer "P3" receives
      | Event     | From |
      | Connected | P0   |
      | Connected | P1   |
      | Connected | P2   |
    And the peer "P2" receives
      | Event     | From |
      | Connected | P3   |
    And the peer "P1" receives
      | Event     | From |
      | Connected | P3   |
    When the peer "P2" disconnects
    Then the peer "P0" receives
      | Event        | From |
      | Disconnected | P2   |
    Then the peer "P1" receives
      | Event        | From |
      | Disconnected | P2   |
    And the peer "P3" receives
      | Event        | From |
      | Disconnected | P2   |

  Scenario: Connection only with specifics peers
    Given the following peers are started
      | Name       | Port |
      | P0 | 9000 |
    When the following peers connect to "P0"
      | Name | Port | Authorize connection with |
      | P1   | 9001 | P2                        |
      | P2   | 9002 | P1                        |
      | P3   | 9003 | P4,P5                     |
      | P4   | 9004 | P3,P5                     |
      | P5   | 9005 | P3,P4                     |
    Then the peer "P1" receives
      | Event     | From |
      | Connected | P2   |
    And the peer "P1" does not receives
      | Event     | From |
      | Connected | P3   |
      | Connected | P4   |
      | Connected | P5   |
    And the peer "P2" receives
      | Event     | From |
      | Connected | P1   |
    And the peer "P2" does not receives
      | Event     | From |
      | Connected | P3   |
      | Connected | P4   |
      | Connected | P5   |
    And the peer "P3" receives
      | Event     | From |
      | Connected | P4   |
      | Connected | P5   |
    And the peer "P3" does not receives
      | Event     | From |
      | Connected | P1   |
      | Connected | P2   |
    And the peer "P4" receives
      | Event     | From |
      | Connected | P3   |
      | Connected | P5   |
    And the peer "P4" does not receives
      | Event     | From |
      | Connected | P1   |
      | Connected | P2   |
    And the peer "P5" receives
      | Event     | From |
      | Connected | P3   |
      | Connected | P4   |
    And the peer "P5" does not receives
      | Event     | From |
      | Connected | P1   |
      | Connected | P2   |
