const crypto = require('crypto');
const WebSocket = require('ws');
const wss = new WebSocket.Server({ port: 8080 });
console.log('WebSocket server is running on ws://localhost:8080');
function verifySignature(message, signature, key) {
  const expectedSignature = createSignature(message, key);
  return crypto.timingSafeEqual(
    Buffer.from(signature, 'hex'),
    Buffer.from(expectedSignature, 'hex')
  );
}
wss.on('connection', (ws) => {
  console.log('New client connected');
  const randomBytes = crypto.randomBytes(16);
  const randomValue = randomBytes.reduce((acc, byte, i) => {
    return acc + byte * (256 ** i);
  }, 0);
  const secretKey = randomValue;
  const hmac = crypto.createHmac('sha256', secretKey);
  hmac.update('data');
  const hmacDigest = hmac.digest('hex');
  ws.send(secretKey);
  ws.send(hmacDigest);
  ws.send("data");
  ws.on('close', () => {
    console.log('Client disconnected');
  });
});
