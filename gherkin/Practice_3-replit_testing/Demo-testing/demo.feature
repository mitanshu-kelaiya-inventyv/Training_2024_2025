@ignore
Feature: Login functionality

  Scenario: User logs in with valid credentials
    Given I open the login page
    When I enter valid credentials
    Then I should be redirected to the home page

  Scenario: User logs in with invalid credentials
    Given I open the login page
    When I enter invalid credentials
    Then I should see an error message
