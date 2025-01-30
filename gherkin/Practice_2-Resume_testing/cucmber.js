// cucumber.js
module.exports = {
    default: {
      require: ["features/step-definitions/*.js"],
      format: ["progress-bar", "json:reports/cucumber-report.json"],
      paths: ["features/*.feature"],
    }
  };
  // module.exports = {
  //   default: "--require step-definitions/*.js --require support/*.js --format json:reports/cucumber-report.json"
  // };
  