Feature: User login functionality on Replit

  Background:
    Given the user is on the LearnXinYMinutes homepage

  Scenario: Successful login with valid credentials
    Given the user navigates to the login page
    When the user enters a valid "username"
    And the user enters the correct "pass"
    When the user clicks the "Log In" button
    Then the user should be redirected to the Replit dashboard


