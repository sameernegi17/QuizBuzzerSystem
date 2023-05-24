// Pong game implementation

// Get canvas and context
const canvas = document.getElementById("pong-container") as HTMLCanvasElement;
const context = canvas.getContext("2d");

// Set canvas dimensions
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

// Paddle properties
const paddleWidth = 10;
const paddleHeight = 100;
const paddleMargin = 10;
const paddleSpeed = 10;

// Create paddles
const leftPaddle = {
  x: paddleMargin,
  y: canvas.height / 2 - paddleHeight / 2,
  width: paddleWidth,
  height: paddleHeight,
  dy: 0
};

const rightPaddle = {
  x: canvas.width - paddleWidth - paddleMargin,
  y: canvas.height / 2 - paddleHeight / 2,
  width: paddleWidth,
  height: paddleHeight,
  dy: 0
};

// Ball properties
const ballSize = 10;
const ballSpeed = 4;

const ball = {
  x: canvas.width / 2,
  y: canvas.height / 2,
  dx: ballSpeed,
  dy: ballSpeed
};

// Counter properties
let counter = 0;

// Update paddles' position based on mouse movement
document.addEventListener("mousemove", function (event) {
  const rect = canvas.getBoundingClientRect();
  const mouseY = event.clientY - rect.top;

  leftPaddle.y = mouseY - paddleHeight / 2;
  rightPaddle.y = mouseY - paddleHeight / 2;
});

// Update game objects
function update() {
  // Move paddles
  leftPaddle.y += leftPaddle.dy;
  rightPaddle.y += rightPaddle.dy;

  // Keep paddles within canvas bounds
  if (leftPaddle.y < 0) leftPaddle.y = 0;
  if (leftPaddle.y + leftPaddle.height > canvas.height) leftPaddle.y = canvas.height - leftPaddle.height;
  if (rightPaddle.y < 0) rightPaddle.y = 0;
  if (rightPaddle.y + rightPaddle.height > canvas.height) rightPaddle.y = canvas.height - rightPaddle.height;

  // Move ball
  ball.x += ball.dx;
  ball.y += ball.dy;

  // Handle ball collision with paddles
  if (
    ball.x - ballSize / 2 < leftPaddle.x + leftPaddle.width &&
    ball.y + ballSize / 2 > leftPaddle.y &&
    ball.y - ballSize / 2 < leftPaddle.y + leftPaddle.height
  ) {
    ball.dx *= -1;
    ball.dx *= 1.1; // Increase ball speed by 10%
    counter++; // Increment counter
    updateCounter(); // Update counter on the webpage
  }

  if (
    ball.x + ballSize / 2 > rightPaddle.x &&
    ball.y + ballSize / 2 > rightPaddle.y &&
    ball.y - ballSize / 2 < rightPaddle.y + rightPaddle.height
  ) {
    ball.dx *= -1;
    ball.dx *= 1.1; // Increase ball speed by 10%
    counter++; // Increment counter
    updateCounter(); // Update counter on the webpage
  }

  // Handle ball collision with top and bottom walls
  if (ball.y - ballSize / 2 < 0 || ball.y + ballSize / 2 > canvas.height) {
    ball.dy *= -1;
  }

  // Handle ball going off-screen (left or right)
  if (ball.x - ballSize / 2 < 0 || ball.x + ballSize / 2 > canvas.width) {
    // Reset ball position
    ball.x = canvas.width / 2;
    ball.y = canvas.height / 2;
    counter = 0; // Reset counter
    ball.dx = ballSpeed * (Math.random() > 0.5 ? 1 : -1); // Random starting direction
    ball.dy = ballSpeed * (Math.random() > 0.5 ? 1 : -1); // Random starting direction
    updateCounter(); // Update counter on the webpage
  }
}

// Render game objects
function draw() {
  // Clear canvas
  context.clearRect(0, 0, canvas.width, canvas.height);

  // Draw paddles
  context.fillStyle = "#b3b3b3";
  context.fillRect(leftPaddle.x, leftPaddle.y, leftPaddle.width, leftPaddle.height);
  context.fillRect(rightPaddle.x, rightPaddle.y, rightPaddle.width, rightPaddle.height);

  // Draw ball
  context.beginPath();
  context.arc(ball.x, ball.y, ballSize / 2, 0, Math.PI * 2);
  context.fillStyle = "#b3b3b3";
  context.fill();
  context.closePath();
}

// Update counter on the webpage
function updateCounter() {
  const counterElement = document.getElementById("counter");
  counterElement.textContent = counter.toString();
}

// Game loop
function gameLoop() {
  update();
  draw();
  requestAnimationFrame(gameLoop);
}

// Start the game loop
gameLoop();
