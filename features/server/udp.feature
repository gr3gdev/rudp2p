Feature: UDP Server

  Scenario: Send text to another server
    Given the following servers are started
    | Name | Port |
    | S1   | 9001 |
    | S2   | 9002 |
    When the server "S1" sends "Hello I am S1" to the server "S2"
    Then the server "S2" receives "Hello I am S1" from the server "S1"
