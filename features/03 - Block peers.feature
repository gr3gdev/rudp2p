Feature: Block peers

  Background:
    Given the following peers are started
      | Name | Port |
      | P0   | 9000 |
    When the following peers connect to "P0"
      | Name | Port | Authorize connection with |
      | P1   | 9001 | P0,P2                     |
    Then the peer "P1" receives
      | Event     | From |
      | Connected | P0   |
    Then the peer "P0" receives
      | Event     | From |
      | Connected | P1   |

  Scenario: Block an authorized peer
    When the peer "P1" blocks the peer "P2"
    And the following peers connect to "P0"
      | Name | Port | Authorize connection with |
      | P2   | 9002 | P0,P1                     |
    Then the peer "P2" receives
      | Event     | From |
      | Connected | P0   |
    And the peer "P2" does not receives
      | Event     | From |
      | Connected | P1   |
    When the peer "P1" sends "How are you ?" to all
    Then the peer "P0" receives
      | Event         | From |
      | How are you ? | P1   |
    And the peer "P2" does not receives
      | Event         | From |
      | How are you ? | P1   |

  Scenario: A block peer attempt to connect
    When the peer "P1" blocks the peer "P2"
    And the following peers connect to "P1"
      | Name | Port |
      | P2   | 9002 |
    Then the peer "P2" does not receives
      | Event     | From |
      | Connected | P1   |
