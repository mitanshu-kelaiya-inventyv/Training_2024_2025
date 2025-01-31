import { promisify } from 'util';
import { exec } from 'child_process';
import express from 'express'
const app = express();
const port = process.env.port || '4000';

const execPromise = promisify(exec);

app.get('/', (req, res)=>{
    res.send("Server started")
});


app.get('/run-test', async (req, res) => {
    try {
        // Run Cucumber tests
        const { stdout: cucumberOutput, stderr: cucumberError } = await execPromise('npm run test');
        console.log("Cucumber Test Output:", cucumberOutput);

        if (cucumberError) {
            console.error(`Cucumber Test Stderr: ${cucumberError}`);
        }

        // Run Report Generation
        const { stdout: cucumberReport, stderr: cucumberReportError } = await execPromise('npm run generate_report');
        console.log("Cucumber Report Output:", cucumberReport);

        if (cucumberReportError) {
            console.error(`Report Generation Stderr: ${cucumberReportError}`);
        }

        return res.json({
            message: 'Tests executed successfully with report',
            testOutput: cucumberOutput,
            reportOutput: cucumberReport
        });

    } catch (error) {
        console.error(`Error: ${error.message}`);
        return res.status(500).json({ message: 'Test execution failed', error: error.message });
    }
});

app.listen(port, ()=>{
    console.log(`Server listening at http://localhost:${port}`);
})