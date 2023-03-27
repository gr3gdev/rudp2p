Feature: Exchange messages

  Scenario: Send a text to all peers
    Given the following peers are started
      | Name       | Port |
      | Dispatcher | 9100 |
    When the following peers connect to "Dispatcher"
      | Name | Port |
      | P1   | 9101 |
      | P2   | 9102 |
      | P3   | 9103 |
    Then the peer "P2" receives
      | Event     | From |
      | Connected | P1   |
    And the peer "P3" receives
      | Event     | From |
      | Connected | P1   |
    When the peer "P1" sends "Hello all" to all
    Then the peer "P2" receives
      | Event     | From |
      | Hello all | P1   |
    And the peer "P3" receives
      | Event     | From |
      | Hello all | P1   |

  Scenario: Send a file to a peer
    Given the following peers are started
      | Name       | Port |
      | Dispatcher | 9200 |
    When the following peers connect to "Dispatcher"
      | Name | Port |
      | P1   | 9201 |
      | P2   | 9202 |
      | P3   | 9203 |
    Then the peer "P2" receives
      | Event     | From |
      | Connected | P1   |
    And the peer "P3" receives
      | Event     | From |
      | Connected | P1   |
    When the peer "P2" sends "file:/tests/test.txt" to "P1"
    Then the peer "P1" receives
      | Event                | From |
      | file:/tests/test.txt | P2   |