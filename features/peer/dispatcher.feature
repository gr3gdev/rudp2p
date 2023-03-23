Feature: Dispatcher

  Scenario: Connection of peers
    Given a peer "D" is started on port 9000
    When a peer "P1" connects to "D" with the port 9001
    Then the peer "P1" receives "Connected" from "P1"
    When a peer "P2" connects to "D" with the port 9002
    Then the peer "P2" receives "Connected" from "P2"
    And the peer "P2" receives "Connected" from "P1"
    And the peer "P1" receives "Connected" from "P2"
    When a peer "P3" connects to "D" with the port 9003
    Then the peer "P3" receives "Connected" from "P3"
    And the peer "P3" receives "Connected" from "P2"
    And the peer "P3" receives "Connected" from "P1"
    And the peer "P2" receives "Connected" from "P3"
    And the peer "P1" receives "Connected" from "P3"
    When a peer "P2" disconnects
    Then the peer "P1" receives "Disconnected" from "P2"
    And the peer "P3" receives "Disconnected" from "P2"
