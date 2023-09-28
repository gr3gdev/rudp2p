Feature: Exchange messages

  Scenario: A peer sends a text to all peers
    Given the following peers are started
      | Name | Port |
      | P0   | 9300 |
      | P1   | 9301 |
      | P2   | 9302 |
      | P3   | 9303 |
    When the peer "P1" connects to "P0"
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P1   |
    When the peer "P2" connects to "P0"
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P2   |
    When the peer "P3" connects to "P0"
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P3   |
    Then the peer "P2" receives
      | Event     | From |
      | CONNECTED | P0   |
      | CONNECTED | P1   |
      | CONNECTED | P3   |
    And the peer "P3" receives
      | Event     | From |
      | CONNECTED | P0   |
      | CONNECTED | P1   |
      | CONNECTED | P2   |
    When the peer "P1" sends "Hello all" to "all"
    Then the peer "P0" receives
      | Event   | Content   | From |
      | MESSAGE | Hello all | P1   |
    Then the peer "P2" receives
      | Event   | Content   | From |
      | MESSAGE | Hello all | P1   |
    And the peer "P3" receives
      | Event   | Content   | From |
      | MESSAGE | Hello all | P1   |

  Scenario: A peer sends a file to a peer
    Given the following peers are started
      | Name | Port |
      | P0   | 9400 |
      | P1   | 9401 |
      | P2   | 9402 |
      | P3   | 9403 |
    When the peer "P1" connects to "P0"
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P1   |
    When the peer "P2" connects to "P0"
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P2   |
    When the peer "P3" connects to "P0"
    Then the peer "P0" receives
      | Event     | From |
      | CONNECTED | P3   |
    Then the peer "P2" receives
      | Event     | From |
      | CONNECTED | P0   |
      | CONNECTED | P1   |
      | CONNECTED | P3   |
    And the peer "P3" receives
      | Event     | From |
      | CONNECTED | P0   |
      | CONNECTED | P1   |
      | CONNECTED | P2   |
    When the peer "P2" sends "file:/tests/test.txt" to "P1"
    Then the peer "P1" receives
      | Event   | Content              | From |
      | MESSAGE | file:/tests/test.txt | P2   |