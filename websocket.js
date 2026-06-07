const crypto = require('crypto');
const WebSocket = require('ws');
const wss = new WebSocket.Server({ port: 8080 });
console.log('WebSocket server is running on ws://localhost:8080');
wss.on('connection', (ws) => {
  console.log('New client connected');
