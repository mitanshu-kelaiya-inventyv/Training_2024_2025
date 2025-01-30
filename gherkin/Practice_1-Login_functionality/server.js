// server.js
const express = require('express');
const path = require('path');
const app = express();

// Serve static files like CSS, images
app.use(express.static(path.join(__dirname, 'public')));

// Set view engine to EJS
app.set('view engine', 'ejs');

// Middleware to parse POST data
app.use(express.urlencoded({ extended: true }));

// Route to serve login page
app.get('/', (req, res) => {
  res.render('login', { error: null }); // Pass error as null initially
});

// Route to handle login
app.post('/login', (req, res) => {
    const { username, password } = req.body;
    console.log(username,password);
    if (username === 'admin' && password === 'password123') {
        res.redirect('/home');  // Ensure this path exists and serves the correct page
    } else {
        res.render('login', { error: 'Invalid username or password' });
    }
});


// Route to serve home page after successful login
app.get('/home', (req, res) => {
  res.render('home');
});

// Start the server
app.listen(3000, () => {
  console.log('Server running on http://localhost:3000');
});
