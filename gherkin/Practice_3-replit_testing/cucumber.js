export default {
    default: {
      require: ["features/step-definitions/replit-login.js"],
      format: ["progress-bar", "json:reports/cucumber_report.json"],
      paths: ["features/replit-login.feature"],
      tags: "not @ignore",  
    }
  };