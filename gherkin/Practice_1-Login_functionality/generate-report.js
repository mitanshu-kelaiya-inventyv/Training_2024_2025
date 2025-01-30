const reporter = require('cucumber-html-reporter');

const options = {
  theme: 'bootstrap',
  jsonFile: 'reports/cucumber-report.json', // Path to your JSON report
  output: 'reports/cucumber-report.html',    // Where to save the HTML report
  reportSuiteAsScenarios: true,
  launchReport: true,
};

reporter.generate(options);
