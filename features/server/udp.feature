Feature: Test UDP Server

  Scenario: Exchange between two servers
    Given a server "S1" is started on port 9001
    And a server "S2" is started on port 9002
    When "S1" sends "Hello I am S1" to "S2"
    Then "S2" receives "Hello I am S1" from "S1"
