Feature: Messages

  Scenario: Send a text to all peers
    Given a peer "D" is started on port 9100
    When a peer "P1" connects to "D" with the port 9101
    And a peer "P2" connects to "D" with the port 9102
    And a peer "P3" connects to "D" with the port 9103
    Then the peer "P2" receives "Connected" from "P1"
    And the peer "P3" receives "Connected" from "P1"
    When the peer "P1" sends "Hello all" to all
    Then the peer "P2" receives "Hello all" from "P1"
    And the peer "P3" receives "Hello all" from "P1"

  Scenario: Send a file to a peer
    Given a peer "D" is started on port 9200
    When a peer "P1" connects to "D" with the port 9201
    And a peer "P2" connects to "D" with the port 9202
    And a peer "P3" connects to "D" with the port 9203
    Then the peer "P2" receives "Connected" from "P1"
    And the peer "P3" receives "Connected" from "P1"
    When the peer "P2" sends "file:/tests/test.txt" to "P1"
    Then the peer "P1" receives "file:/tests/test.txt" from "P2"