Feature: Login functionality

  Background: 
    Given I open the login page

Scenario Outline: User tries to log in
  When I enter "<user>" and "<pass>"
  Then I should see "<message>"

Examples:
  | user        | pass        | message                      |
  | validUser   | validPass   | Welcome to the Home Page!    |
  | invalidUser | invalidPass | Invalid credentials          |

