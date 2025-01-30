Feature: Page Title Verification

  Scenario: Verify the title of the local server page
    Given I open "http://localhost:8000"
    Then the page title should be "Mitanshu Kelaiya - Home"
    When I navigate to "http://localhost:8000/skill"
    Then the page title should be "Mitanshu Kelaiya - Skill"
