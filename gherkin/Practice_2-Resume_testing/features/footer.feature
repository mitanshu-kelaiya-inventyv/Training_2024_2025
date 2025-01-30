Feature: Footer UI and Links

  Scenario: Verify that the footer is visible on the home page
    Given I am on the "home" page
    Then the footer should be visible

  Scenario: Verify that the footer is visible on the skill page
    Given I am on the "skill" page
    Then the footer should be visible
