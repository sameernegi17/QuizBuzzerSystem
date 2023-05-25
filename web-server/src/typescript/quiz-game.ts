// Code for generating questions (you can customize this part as per your requirements)
const generateQuestions = () => {
  const questionContainer = document.getElementById('question');
  questionContainer.innerText = 'Very very very long long long long Sample Question'; // Replace with your logic to generate questions
  const playerFirst = document.getElementById('player-first');
  playerFirst.textContent = 'Player » NAME « pressed the button first!';
};

// Event listener for the "Generate Questions" button
const generateBtn = document.getElementById('generate-btn');
generateBtn.addEventListener('click', generateQuestions);
