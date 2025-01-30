Feature: Home Page UI and Content

  Scenario: Verify Background Image on Home Page
    Given I navigate to the home page
    Then the background image should load correctly
    And the background image width should be 100vw
    And the opacity of the background image should be 0.7

  Scenario: Verify Personal Section Content
    Given I am on the home page
    Then the personal section should be displayed with profile image
    And the profile image should have a height and width of 200px
    And the profile image should be circular
    And the personal name should be displayed in black
    And the personal title should be displayed in black
    And the "Download CV" button should be visible and clickable